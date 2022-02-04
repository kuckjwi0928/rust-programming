mod variable;
mod datatype;
mod function;
mod flow_control;

fn main() {
    variable::basic();
    variable::shadowed();
    datatype::basic();
    function::another_function(10, 5);
    function::function_block();
    function::five();
    flow_control::if_expression();
    flow_control::let_with_if();
    flow_control::loops();
}
