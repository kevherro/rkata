pub mod pot;

#[cfg(test)]
mod tests {
    use crate::pot::Pot;

    #[test]
    /// Blinds 600-1200, everyone antes 200. You have 45500.
    /// How many blinds do I have?
    fn core_skill_1() {
        let result = 45500 / 1200;
        assert_eq!(result, 37);
    }

    #[test]
    /// Blinds 250-500, the big blind antes 500.
    /// How much is in the pot before any cards are dealt?
    fn core_skill_2() {
        let pot = Pot::new(250, 500, 500);
        let result = pot.size();
        assert_eq!(result, 1250);
    }

    #[test]
    /// Blinds 250-500, the big blind antes 500.
    /// An early position player opens for 1500,
    /// and a middle position player re-raises to 3800.
    /// How much is in the pot?
    fn core_skill_3() {
        let pot = Pot::new(250, 500, 500);
        let result = pot.size() + 1500 + 3800;
        assert_eq!(result, 6550);
    }

    #[test]
    /// Blinds 100-200, I have 14000.
    /// Do I have more or less than a 70BB stack?
    fn core_skill_4() {
        let result = 14000 / 200;
        assert_eq!(result, 70);
    }

    #[test]
    /// Blinds 50-100, I have 28000.
    /// From early position, I open for 250.
    /// A middle position player re-raises to 700.
    /// Everyone else folds, I call.
    /// How much is in the pot?
    fn core_skill_5() {
        let pot = Pot::new(50, 100, 0);
        let result = pot.size() + 250 + 700 + 450;
        assert_eq!(result, 1550);
    }

    #[test]
    /// Blinds 200-400, the big blind antes 400.
    /// A middle position player opens for 825.
    /// I re-raise to 2,100 from the button.
    /// The blinds fold, the raiser calls.
    /// How much is in the pot?
    fn core_skill_6() {
        let pot = Pot::new(200, 400, 400);
        let result = pot.size() + 825 + 2100 + (2100 - 825);
        assert_eq!(result, 5200);
    }

    #[test]
    /// Blinds 200-400, the big blind antes 400.
    /// I have 60000.
    /// Do I have more or less than a 70BB stack?
    fn core_skill_7() {
        let result = 60000 / 400;
        assert!(result > 70);
    }

    #[test]
    /// Blinds 150-300.
    /// I have 20200, one of the shortest stacks at the table!
    /// Do I have more or less than a 70BB stack?
    fn core_skill_8() {
        let result = 20200 / 300;
        assert!(result < 70);
    }

    #[test]
    /// Blinds 150-300.
    /// The first player to act pre-flop (UTG) opens for 800.
    /// I make it 2200 in the next seat.
    /// A late position player calls, and UTG calls.
    /// How much is in the pot?
    fn core_skill_9() {
        let pot = Pot::new(150, 300, 0);
        let result = pot.size() + 800 + 2200 + 2200 + (2200 - 800);
        assert_eq!(result, 7050);
    }

    #[test]
    /// Blinds 600-1200, the big blind antes 1200.
    /// I have 64000.
    /// How many BBs do I have?
    fn core_skill_10() {
        let result = 64000 / 1200;
        assert_eq!(result, 53);
    }

    #[test]
    /// Blinds 800-1600, the big blind antes 1600.
    /// I have 191000.
    /// How many BBs do I have?
    fn core_skill_11() {
        let result = 191000 / 1600;
        assert_eq!(result, 119);
    }

    #[test]
    /// Blinds 800-1600, the big blind antes 1600.
    /// I have 191000.
    /// A late position player opens for 3500.
    /// From the button, I make it 8500.
    /// Everyone folds to the first raiser who raises to 22600.
    /// I call.
    /// How much is in the pot?
    fn core_skill_12() {
        let pot = Pot::new(800, 1600, 1600);
        let result = pot.size() + (22600 * 2);
        assert_eq!(result, 49200);
    }
}
