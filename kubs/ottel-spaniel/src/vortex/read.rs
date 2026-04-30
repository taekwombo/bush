use std::path::Path;

use vortex::array::arrays::Struct;
use vortex::array::{Array, ArrayRef};
use vortex::error::VortexError;
use vortex::expr::*;
use vortex::file::OpenOptionsSessionExt;
use vortex::scalar::Scalar;

use crate::Format;

pub trait AsSpanData {
    fn get_names(&self) -> impl Iterator<Item = Scalar>;
}

impl AsSpanData for Array<Struct> {
    fn get_names(&self) -> impl Iterator<Item = Scalar> {
        struct Iter<'a> {
            arr: &'a Array<Struct>,
            index: usize,
        }

        impl<'a> Iterator for Iter<'a> {
            type Item = Scalar;

            fn next(&mut self) -> Option<Self::Item> {
                if self.index < self.arr.len() {
                    let name = self
                        .arr
                        .scalar_at(self.index)
                        .expect("elem.exists")
                        .as_struct()
                        .field("name")
                        .expect("field.exists");

                    self.index += 1;
                    return Some(name);
                }

                None
            }
        }

        Iter {
            arr: self,
            index: 0,
        }
    }
}

pub struct Read<'a> {
    files: Vec<Box<Path>>,
    index: usize,
    format: &'a Format,
    filter: Option<Expression>,
    projection: Option<Expression>,
    reader: Option<Box<dyn Iterator<Item = Result<ArrayRef, VortexError>> + Send + 'static>>,
}

impl<'a> Read<'a> {
    pub fn new(format: &'a Format, files: Vec<Box<Path>>) -> Self {
        Self {
            files,
            format,
            index: 0,
            filter: None,
            projection: None,
            reader: None,
        }
    }

    pub fn with_filter(mut self, filter: Expression) -> Self {
        self.filter.replace(filter);
        self
    }

    pub fn with_projection(mut self, projection: Expression) -> Self {
        self.projection.replace(projection);
        self
    }

    async fn init_reader(&mut self) {
        let Format::Vortex { session, runtime } = self.format else {
            unreachable!();
        };

        let file = &self.files[self.index];
        let file = session
            .open_options()
            .open_path(file)
            .await
            .expect("vortex.open");

        let mut scan = file.scan().unwrap();

        if let Some(ref filter) = self.filter {
            scan = scan.with_filter(filter.clone());
        }

        if let Some(ref projection) = self.projection {
            scan = scan.with_projection(projection.clone());
        }

        let iter = scan.into_iter(runtime).unwrap();
        let _ = self.reader.replace(Box::new(iter));
    }

    pub async fn next_batch(&mut self) -> Option<Array<Struct>> {
        tracing::info!(idx = self.index, len = self.files.len(), "next_batch");

        while self.index < self.files.len() {
            if self.reader.is_none() {
                self.init_reader().await;
            }

            let next = self.reader.as_mut().expect("reader.exists").next();

            if next.is_none() {
                self.index += 1;
                self.reader = None;
                continue;
            }

            return Some(next.unwrap().expect("vortex.item.ok").downcast::<Struct>());
        }

        None
    }
}

// pub async fn read_unique_span_names(
//     session: &VortexSession,
//     runtime: &CurrentThreadRuntime,
//     time_start: u64,
//     time_end: u64,
// ) -> std::collections::HashSet<String> {
//     use vortex::array::arrays::Struct;
//     use vortex::expr::*;
//
//     let dir = "data-vortex";
//     let filter = and(
//         gt(get_item("time_start", root()), lit(time_start)),
//         lt(get_item("time_end", root()), lit(time_end)),
//     );
//
//     let projection = select(["name"], root());
//     let mut set = std::collections::HashSet::with_capacity(500);
//
//     for file in read_dir(dir) {
//         println!("Reading {}", file);
//         let file = session
//             .open_options()
//             .open_path(format!("{}/{}", dir, file))
//             .await
//             .expect("ok");
//         let scan = file
//             .scan()
//             .unwrap()
//             .with_filter(filter.clone())
//             .with_projection(projection.clone())
//             .with_ordered(true);
//
//         for arr in scan.into_iter(runtime).unwrap() {
//             let arr = arr.unwrap();
//             let struct_arr = arr.downcast::<Struct>();
//
//             for i in 0..struct_arr.len() {
//                 let name = struct_arr
//                     .scalar_at(i)
//                     .unwrap()
//                     .as_struct()
//                     .field("name")
//                     .unwrap();
//                 let name = name.as_utf8().value().unwrap().as_str().to_owned();
//
//                 set.insert(name);
//
//                 if set.len() >= 200 {
//                     return set;
//                 }
//             }
//         }
//     }
//
//     set
// }
