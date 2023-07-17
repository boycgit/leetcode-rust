use std::collections::HashSet;
use std::hash::Hash;

// 定义一个泛型的结构体，可以包含任意类型和长度的数组
#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Array<T, const N: usize>([T; N]);


/// 将一个二维数组转换成一个 HashSet
///
/// # Examples
///
/// ```
/// use std::collections::HashSet;
/// use array_to_hashset::Array;
///
/// // 定义一个 Vec<Vec<i32>> 类型的二维数组
/// let arr = vec![vec![1, 2], vec![1, 3], vec![1, 4], vec![2, 3], vec![2, 4], vec![3, 4]];
///
/// // 调用工具函数，获取 HashSet
/// let set = array_to_hashset(&arr);
///
/// // 打印集合中的元素
/// for x in &set {
///     println!("{:?}", x.0);
/// }
/// ```
pub fn array_to_hashset<T: Eq + Hash + Copy, const N: usize>(arr: impl IntoIterator<Item = impl IntoIterator<Item = T>>) -> HashSet<Array<T, N>> {
    // 将二维数组转换成一个迭代器，并将每个元素转换成 Array 类型
    let iter = arr.into_iter().flatten().map(|x| Array::<T, N>([x.clone(); N]));

    // 使用 collect 方法来从迭代器创建 HashSet
    let set: HashSet<_> = iter.collect();

    // 返回 HashSet
    set
}

// 这里为泛型参数 T 添加了 Eq + Hash + Copy 的 trait bound，这是为了保证 T 类型可以作为 HashSet 的元素，并且可以在 map 函数中复制。你也可以使用 Clone trait 来替代 Copy trait，但是需要在 map 函数中显式调用 clone 方法