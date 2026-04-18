use vortex::file::OpenOptionsSessionExt;
use vortex::io::runtime::current::CurrentThreadRuntime;
use vortex::session::VortexSession;

use crate::misc::read_dir;

pub async fn read_unique_span_names(
    session: &VortexSession,
    runtime: &CurrentThreadRuntime,
    time_start: u64,
    time_end: u64,
) -> std::collections::HashSet<String> {
    use vortex::array::arrays::Struct;
    use vortex::expr::*;

    let dir = "data-vortex";
    let filter = and(
        gt(get_item("time_start", root()), lit(time_start)),
        lt(get_item("time_end", root()), lit(time_end)),
    );

    let projection = select(["name"], root());
    let mut set = std::collections::HashSet::with_capacity(500);

    for file in read_dir(dir) {
        println!("Reading {}", file);
        let file = session
            .open_options()
            .open_path(format!("{}/{}", dir, file))
            .await
            .expect("ok");
        let scan = file
            .scan()
            .unwrap()
            .with_filter(filter.clone())
            .with_projection(projection.clone())
            .with_ordered(true);

        for arr in scan.into_iter(runtime).unwrap() {
            let arr = arr.unwrap();
            let struct_arr = arr.downcast::<Struct>();

            for i in 0..struct_arr.len() {
                let name = struct_arr
                    .scalar_at(i)
                    .unwrap()
                    .as_struct()
                    .field("name")
                    .unwrap();
                let name = name.as_utf8().value().unwrap().as_str().to_owned();

                set.insert(name);

                if set.len() >= 200 {
                    return set;
                }
            }
        }
    }

    set
}
