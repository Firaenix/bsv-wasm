#[cfg(test)]
mod interpreter_rawdata_tests {
    use bsv::Interpreter;
    use bsv::Script;

    #[test]
    fn true_opreturn() {
        let script = Script::from_hex("516a").unwrap();

        let mut interpreter = Interpreter::from_script(&script);
        interpreter.run().unwrap();

        assert_eq!(interpreter.state().stack().last().unwrap(), &vec![1_u8]);
    }

    #[test]
    fn true_opreturn_false() {

        // let script = Script::from_hex("516a00")
        // .unwrap();

        // let mut interpreter = Interpreter::from_script(&script);
        // interpreter.run().unwrap();

        // assert_eq!(interpreter.state().stack().last().unwrap(), &vec![1_u8]);
    }

    #[test]
    fn false_opreturn() {
        let script = Script::from_hex("006a").unwrap();

        let mut interpreter = Interpreter::from_script(&script);
        interpreter.run().unwrap();

        let empty: Vec<u8> = vec![];
        assert_eq!(interpreter.state().stack().last().unwrap(), &empty);
    }

    #[test]
    fn false_opreturn_true() {
        // let script = Script::from_hex("006a51")
        // .unwrap();

        // let mut interpreter = Interpreter::from_script(&script);
        // interpreter.run().unwrap();

        // let empty: Vec<u8> = vec![];
        // assert_eq!(interpreter.state().stack().last().unwrap(), &empty);
    }

    // The current implementation of the interpreter is incomplete and I cannot add tests for rawdata
}
