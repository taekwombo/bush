export interface Base<T> {
    optional?: boolean;
    defaultValue?: T;
}

export interface Names {
    name: string;
    shortName?: string;
}

export interface Description {
    description?: string;
}

export interface Generic<T> extends Base<T>, Names, Description {}

export type InferK<O extends Names> = O['name'];
export type InferV<T, O extends Base<T>> = O['optional'] extends true
    ? O['defaultValue'] extends T
        ? T
        : T | null
    : T
    ;

