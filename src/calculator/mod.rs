pub fn calculate_rpn(input: String) -> String {
  let mut stack: Vec<i32> = vec![];
  let operators = vec!["+", "-", "*", "/"];

  for c in input.split_ascii_whitespace() {
    if operators.iter().find(|&&x| x == c) != None {
      let first = match stack.pop() {
        Some(number) => number,
        None => return "Error: operator operand mismatch".to_string(),
      };
      let second = match stack.pop() {
        Some(number) => number,
        None => return "Error: operator operand mismatch".to_string(),
      };

      stack.push(calc(first, second, c));

      continue;
    }
    match c.parse::<i32>() {
      Ok(number) => {
        stack.push(number);
      }
      Err(_) => return format!("Error: { } non supported characters", c).to_string(),
    }
  }
  if stack.len() != 1 {
    return "Error: operator operand mismatch".to_string();
  }
  stack.pop().unwrap().to_string()
}

fn calc(first: i32, second: i32, operator: &str) -> i32 {
  match operator {
    "+" => first + second,
    "-" => first - second,
    "*" => first * second,
    "/" => first / second,
    _ => 0,
  }
}
