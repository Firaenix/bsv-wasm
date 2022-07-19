#[cfg(test)]
mod interpreter_arithmetic_tests {
    use bsv::Script;
    use bsv::Interpreter;

    #[test]
    fn one_plus_one_equals_two() {
        let script = Script::from_asm_string(r#"
            OP_1
            OP_1
            OP_ADD
        "#).unwrap();

        let mut interpreter = Interpreter::from_script(&script);
        interpreter.run().unwrap();
         

        assert_eq!(interpreter.state().stack().last().unwrap(), &vec![2_u8]);
    }

    #[test]
    fn one_plus_one_equals_verify() {
        let script = Script::from_asm_string(r#"
            OP_1
            OP_1
            OP_ADD
            OP_2
            OP_EQUALVERIFY
        "#).unwrap();

        let mut interpreter = Interpreter::from_script(&script);
        interpreter.run().unwrap();
         

        assert_eq!(interpreter.state().stack().last(), None);
    }


    #[test]
    fn one_times_one_equals_one() {
        let script = Script::from_asm_string(r#"
            OP_1
            OP_1
            OP_MUL
            OP_1
            OP_EQUAL
        "#).unwrap();

        let mut interpreter = Interpreter::from_script(&script);
        interpreter.run().unwrap();
         

        assert_eq!(interpreter.state().stack().last().unwrap(), &vec![1_u8]);
    }

    #[test]
    fn two_times_two_equals_four() {
        let script = Script::from_asm_string(r#"
            OP_2
            OP_2
            OP_MUL
            OP_4
            OP_EQUAL
        "#).unwrap();

        let mut interpreter = Interpreter::from_script(&script);
        interpreter.run().unwrap();
         
        assert_eq!(interpreter.state().stack().last().unwrap(), &vec![1_u8]);
    }

}
