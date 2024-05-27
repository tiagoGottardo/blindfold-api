use crate::handlers::game::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_1_make_san_move() {
        assert_eq!(
            make_san_move(
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
                "d3"
            )
            .unwrap(),
            "rnbqkbnr/pppppppp/8/8/8/3P4/PPP1PPPP/RNBQKBNR b KQkq - 0 1"
        );
    }

    #[test]
    fn t_2_make_san_move() {
        assert_eq!(
            make_san_move(
                "rnbqkbnr/pppppppp/8/8/8/3P4/PPP1PPPP/RNBQKBNR b KQkq - 0 1",
                "a6"
            )
            .unwrap(),
            "rnbqkbnr/1ppppppp/p7/8/8/3P4/PPP1PPPP/RNBQKBNR w KQkq - 0 1"
        );
    }

    #[test]
    fn t_3_make_san_move() {
        assert_eq!(
            make_san_move(
                "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
                "Pe2e4"
            )
            .unwrap(),
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1"
        );
    }

    #[test]
    fn t_1_make_lan_move() {
        assert_eq!(
            make_lan_move("3k4/8/8/8/8/8/4R3/4K3 w - - 0 1", "Re2a2").unwrap(),
            (
                "Ra2".to_string(),
                "3k4/8/8/8/8/8/R7/4K3 b - - 0 1".to_string()
            )
        );
    }

    // //3k4/p7/8/8/8/8/R7/4K3 b - - 0 1
    // #[test]
    // fn t_2_make_lan_move() {
    //     assert_eq!(
    //         make_lan_move("3k4/8/8/8/8/8/R7/4K3 b - - 0 1", "d8c8").unwrap(),
    //         (
    //             "Kc8".to_string(),
    //             "2k5/8/8/8/8/8/R7/4K3 w - - 0 1".to_string()
    //         )
    //     );
    // }
}
