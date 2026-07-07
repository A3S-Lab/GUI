use a3s_gui::{GuiError, GuiResult};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Operator {
    pub fn from_symbol(symbol: &str) -> GuiResult<Self> {
        match symbol {
            "+" => Ok(Self::Add),
            "-" => Ok(Self::Subtract),
            "*" => Ok(Self::Multiply),
            "/" => Ok(Self::Divide),
            other => Err(GuiError::host(format!(
                "unknown calculator operator {other:?}"
            ))),
        }
    }

    pub fn symbol(self) -> &'static str {
        match self {
            Self::Add => "+",
            Self::Subtract => "-",
            Self::Multiply => "*",
            Self::Divide => "/",
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CalculatorState {
    display: String,
    history: String,
    accumulator: Option<f64>,
    pending_operator: Option<Operator>,
    waiting_for_operand: bool,
    has_error: bool,
}

impl Default for CalculatorState {
    fn default() -> Self {
        Self {
            display: "0".to_string(),
            history: String::new(),
            accumulator: None,
            pending_operator: None,
            waiting_for_operand: false,
            has_error: false,
        }
    }
}

impl CalculatorState {
    pub fn display(&self) -> &str {
        &self.display
    }

    pub fn history(&self) -> &str {
        &self.history
    }

    pub fn has_error(&self) -> bool {
        self.has_error
    }

    pub fn press_digit(&mut self, digit: String) -> GuiResult<()> {
        self.recover_from_error();
        if digit.len() != 1 || !digit.as_bytes()[0].is_ascii_digit() {
            return Err(GuiError::host(format!(
                "invalid calculator digit {digit:?}"
            )));
        }

        if self.waiting_for_operand || self.display == "0" {
            self.display = digit;
            self.waiting_for_operand = false;
            return Ok(());
        }

        if self.display.len() < 16 {
            self.display.push_str(&digit);
        }
        Ok(())
    }

    pub fn press_decimal(&mut self) {
        self.recover_from_error();
        if self.waiting_for_operand {
            self.display = "0.".to_string();
            self.waiting_for_operand = false;
        } else if !self.display.contains('.') {
            self.display.push('.');
        }
    }

    pub fn press_operator(&mut self, symbol: String) -> GuiResult<()> {
        self.recover_from_error();
        let operator = Operator::from_symbol(&symbol)?;
        let current = self.current_value()?;

        if let (Some(accumulator), Some(pending)) = (self.accumulator, self.pending_operator) {
            if !self.waiting_for_operand {
                if pending == Operator::Divide && current == 0.0 {
                    self.set_error("Cannot divide by zero");
                    return Ok(());
                }
                let value = apply_operator(accumulator, current, pending)?;
                self.accumulator = Some(value);
                self.display = format_number(value);
            }
        } else {
            self.accumulator = Some(current);
        }

        self.pending_operator = Some(operator);
        self.waiting_for_operand = true;
        self.history = format!("{} {}", self.display, operator.symbol());
        Ok(())
    }

    pub fn press_equals(&mut self) -> GuiResult<()> {
        self.recover_from_error();
        let Some(operator) = self.pending_operator else {
            self.history = format!("{} =", self.display);
            self.waiting_for_operand = true;
            return Ok(());
        };

        let left = self.accumulator.unwrap_or(0.0);
        let right = self.current_value()?;
        if operator == Operator::Divide && right == 0.0 {
            self.set_error("Cannot divide by zero");
            return Ok(());
        }
        let result = apply_operator(left, right, operator)?;
        self.history = format!(
            "{} {} {} =",
            format_number(left),
            operator.symbol(),
            format_number(right)
        );
        self.display = format_number(result);
        self.accumulator = None;
        self.pending_operator = None;
        self.waiting_for_operand = true;
        Ok(())
    }

    pub fn clear(&mut self) {
        *self = Self::default();
    }

    pub fn clear_entry(&mut self) {
        self.display = "0".to_string();
        self.has_error = false;
        self.waiting_for_operand = false;
    }

    pub fn backspace(&mut self) {
        if self.has_error || self.waiting_for_operand {
            self.clear_entry();
            return;
        }

        self.display.pop();
        if self.display.is_empty() || self.display == "-" {
            self.display = "0".to_string();
        }
    }

    pub fn percent(&mut self) -> GuiResult<()> {
        self.recover_from_error();
        let value = self.current_value()? / 100.0;
        self.display = format_number(value);
        Ok(())
    }

    pub fn reciprocal(&mut self) -> GuiResult<()> {
        self.recover_from_error();
        let value = self.current_value()?;
        if value == 0.0 {
            self.set_error("Cannot divide by zero");
            return Ok(());
        }
        self.history = format!("1/({})", self.display);
        self.display = format_number(1.0 / value);
        self.waiting_for_operand = true;
        Ok(())
    }

    pub fn square(&mut self) -> GuiResult<()> {
        self.recover_from_error();
        let value = self.current_value()?;
        self.history = format!("sqr({})", self.display);
        self.display = format_number(value * value);
        self.waiting_for_operand = true;
        Ok(())
    }

    pub fn square_root(&mut self) -> GuiResult<()> {
        self.recover_from_error();
        let value = self.current_value()?;
        if value < 0.0 {
            self.set_error("Invalid input");
            return Ok(());
        }
        self.history = format!("sqrt({})", self.display);
        self.display = format_number(value.sqrt());
        self.waiting_for_operand = true;
        Ok(())
    }

    pub fn toggle_sign(&mut self) {
        self.recover_from_error();
        if self.display == "0" {
            return;
        }
        if let Some(value) = self.display.strip_prefix('-') {
            self.display = value.to_string();
        } else {
            self.display = format!("-{}", self.display);
        }
    }

    fn current_value(&self) -> GuiResult<f64> {
        self.display.parse::<f64>().map_err(|error| {
            GuiError::host(format!(
                "calculator display {:?} is not numeric: {error}",
                self.display
            ))
        })
    }

    fn recover_from_error(&mut self) {
        if self.has_error {
            self.clear();
        }
    }

    fn set_error(&mut self, message: &str) {
        self.display = message.to_string();
        self.history.clear();
        self.accumulator = None;
        self.pending_operator = None;
        self.waiting_for_operand = true;
        self.has_error = true;
    }
}

fn apply_operator(left: f64, right: f64, operator: Operator) -> GuiResult<f64> {
    match operator {
        Operator::Add => Ok(left + right),
        Operator::Subtract => Ok(left - right),
        Operator::Multiply => Ok(left * right),
        Operator::Divide => {
            if right == 0.0 {
                Err(GuiError::host("Cannot divide by zero"))
            } else {
                Ok(left / right)
            }
        }
    }
}

fn format_number(value: f64) -> String {
    if !value.is_finite() {
        return "Error".to_string();
    }
    if value == 0.0 {
        return "0".to_string();
    }
    if value.fract() == 0.0 && value.abs() < 1_000_000_000_000_000.0 {
        return format!("{value:.0}");
    }

    let formatted = format!("{value:.12}");
    formatted
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string()
}
