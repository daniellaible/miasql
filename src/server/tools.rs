use crate::database::datatype::DataType;

pub fn clean_string(mut input: String) -> String {
    input = input.replace("\",","");
    input = input.replace('"',"");
    input = input.as_str().trim().to_string();
    input
}

pub fn datatype_to_string_uppercase(input: &DataType) -> String{
    let mut value = input.to_string().to_uppercase();
    value = value.replace('"',"");
    value
}

pub fn remove_double_slash(mut input: String) -> String{
    let mut success = false;
    while !success {
        input = input.replace(r"\\", r"\".to_string().as_str());
        if !input.contains(r"\\"){
            success = true;
        }
    }
    input
}