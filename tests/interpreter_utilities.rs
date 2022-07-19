#[cfg(test)]
mod interpreter_utility_tests {
    use bsv::Script;
    use bsv::Interpreter;

    #[test]
    fn simple_op_num2bin_test() {
        let script = Script::from_asm_string(r#"
            ABCDEF4243 OP_5
            OP_NUM2BIN ABCDEF4243 OP_EQUAL"#).unwrap();

        let mut interpreter = Interpreter::from_script(&script);
        interpreter.run().unwrap();
         

        assert_eq!(interpreter.state().stack().last().unwrap(), &vec![1_u8]);
    }

    #[test]
    fn num2bin_output_test() {
        let script = Script::from_asm_string(r#"
            ABCDEF4243 OP_5
            OP_NUM2BIN"#).unwrap();

        let mut interpreter = Interpreter::from_script(&script);
        interpreter.run().unwrap();
         

        assert_eq!(hex::encode(interpreter.state().stack().last().unwrap()), "abcdef4243");
    }

    #[test]
    fn num2bin_zero_length_test() {
        let script = Script::from_asm_string(r#"
            ABCDEF4243 OP_0
            OP_NUM2BIN"#).unwrap();

        let mut interpreter = Interpreter::from_script(&script);
        assert!(interpreter.run().is_err());
    }

    #[test]
    fn num2bin_oversized_length_test() {
        let script = Script::from_asm_string(r#"
            ABCDEF4243 OP_16
            OP_NUM2BIN"#).unwrap();

        let mut interpreter = Interpreter::from_script(&script);
        interpreter.run().unwrap();

        assert_eq!(hex::encode(interpreter.state().stack().last().unwrap()), "abcdef42430000000000000000000000");
    }

    #[test]
    fn num2bin_negative_number_test() {
        let script = Script::from_asm_string(r#"
            21e8 OP_NEGATE OP_2 OP_NUM2BIN"#).unwrap();

        let mut interpreter = Interpreter::from_script(&script);
        interpreter.run().unwrap();

        assert_eq!(hex::encode(interpreter.state().stack().last().unwrap()), "2168");
    }

    #[test]
    fn num2bin_oversized_negative_number_test() {
        let script = Script::from_asm_string(r#"
            21e8 OP_NEGATE OP_16 OP_NUM2BIN"#).unwrap();

        let mut interpreter = Interpreter::from_script(&script);
        interpreter.run().unwrap();

        assert_eq!(hex::encode(interpreter.state().stack().last().unwrap()), "21680000000000000000000000000000");
    }
}
