pub fn maximum_beauty(mut items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
    items.sort_unstable();
    items.dedup_by(|right, left| right[1] <= left[1]);
    queries
        .into_iter()
        .map(|q| match items.partition_point(|it| it[0] <= q) {
            0 => 0,
            i => items[i - 1][1],
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::maximum_beauty;

    #[test]
    fn query_based_best_price_per_value() {
        assert_eq!(
            vec![2, 4, 5, 5, 6, 6],
            maximum_beauty(
                vec![vec![1, 2], vec![3, 2], vec![2, 4], vec![5, 6], vec![3, 5]],
                vec![1, 2, 3, 4, 5, 6]
            )
        );

        assert_eq!(
            vec![
                962, 962, 962, 962, 746, 962, 962, 962, 946, 962, 962, 919, 746, 746, 962, 962,
                962, 919, 962
            ],
            maximum_beauty(
                vec![
                    vec![193, 732],
                    vec![781, 962],
                    vec![864, 954],
                    vec![749, 627],
                    vec![136, 746],
                    vec![478, 548],
                    vec![640, 908],
                    vec![210, 799],
                    vec![567, 715],
                    vec![914, 388],
                    vec![487, 853],
                    vec![533, 554],
                    vec![247, 919],
                    vec![958, 150],
                    vec![193, 523],
                    vec![176, 656],
                    vec![395, 469],
                    vec![763, 821],
                    vec![542, 946],
                    vec![701, 676]
                ],
                vec![
                    885, 1445, 1580, 1309, 205, 1788, 1214, 1404, 572, 1170, 989, 265, 153, 151,
                    1479, 1180, 875, 276, 1584
                ]
            )
        )
    }

    #[test]
    fn yields_zero_when_no_prices_are_less_than_query() {
        assert_eq!(vec![0], maximum_beauty(vec![vec![10, 1000]], vec![5]))
    }
}
