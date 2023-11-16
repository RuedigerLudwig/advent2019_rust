// https://stackoverflow.com/a/28392068
#[macro_export]
macro_rules! hashmap {
    () => {
        ::std::collections::HashMap::new()
    };

    ($( $key: expr => $val: expr ),+ $(,)?) => {{
        let mut map = ::std::collections::HashMap::new();
        $( map.insert($key, $val); )+
        map
    }}
}

#[macro_export]
macro_rules! hashset {
    () => {
        ::std::collections::HashSet::new()
    };

    ($( $key: expr ),+ $(,)?) => {{
        let mut set = ::std::collections::HashSet::new();
        $( set.insert($key); )+
        set
    }}
}
