use serde::{Deserialize, Serialize};
use std::{
    convert::From,
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
    vec,
};

//complex number implementation
#[derive(Copy, Clone, Debug, Default, PartialEq)]
struct Complex {
    re: f64,
    im: f64,
}

#[derive(Serialize, Deserialize)]
pub struct ComplexWithString {
    re: f64,
    im: f64,
    string: String,
}

impl Add for Complex {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            re: self.re + other.re,
            im: self.im + other.im,
        }
    }
}

impl AddAssign for Complex {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl Sub for Complex {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            re: self.re - other.re,
            im: self.im - other.im,
        }
    }
}

impl SubAssign for Complex {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            re: self.re * other.re - self.im * other.im,
            im: self.re * other.im + self.im * other.re,
        }
    }
}

impl MulAssign for Complex {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}

impl Div for Complex {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        let div = other.re * other.re - other.im * other.im;
        Self {
            re: (self.re * other.re + self.im * other.im) / div,
            im: (self.im * other.re - self.re * other.im) / div,
        }
    }
}

impl DivAssign for Complex {
    fn div_assign(&mut self, other: Self) {
        *self = *self / other;
    }
}

impl Eq for Complex {}

impl PartialOrd for Complex {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        (self.re * self.re + self.im * self.im)
            .partial_cmp(&(other.re * other.re + other.im * other.im))
    }
}

impl Ord for Complex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.re * self.re + self.im * self.im)
            .total_cmp(&(other.re * other.re + other.im * other.im))
    }
}

impl Display for Complex {
    fn fmt(self: &Complex, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let precision = f.precision().unwrap_or(2);
        let pow_of_10 = 10.0_f64.powi(precision as i32);
        let val = Complex {
            re: (self.re * pow_of_10).round() / pow_of_10,
            im: (self.im * pow_of_10).round() / pow_of_10,
        };
        if val.re == 0.0 && val.im == 0.0 {
            f.write_fmt(format_args!("0"))
        } else if val.im == 0.0 {
            f.write_fmt(format_args!("{}", val.re))
        } else if val.re == 0.0 {
            if val.im == 1.0 {
                f.write_fmt(format_args!("i"))
            } else if val.im == -1.0 {
                f.write_fmt(format_args!("-i"))
            } else {
                f.write_fmt(format_args!("{}i", val.im))
            }
        } else {
            if val.im == 1.0 {
                f.write_fmt(format_args!("{}+i", val.re))
            } else if val.im == -1.0 {
                f.write_fmt(format_args!("{}-i", val.re))
            } else {
                f.write_fmt(format_args!("{}{:+}i", val.re, val.im))
            }
        }
    }
}

impl From<f64> for Complex {
    fn from(val: f64) -> Self {
        Complex { re: val, im: 0.0 }
    }
}

impl From<&str> for Complex {
    //Complex number sum parsing (todo if I care enough: error handling)
    fn from(mut string: &str) -> Self {
        let mut ok = &true;
        if string.is_empty() {
            return Complex { re: 1.0, im: 0.0 };
        }
        let mut sign: f64 = 1.0;
        if string.starts_with("-") {
            sign = -1.0;
            string = string.strip_prefix("-").unwrap();
        } else if string.starts_with("+") {
            string = string.strip_prefix("+").unwrap();
        }
        let val: f64;
        if !string.starts_with("i") {
            let unparsed_val = string.split(&['+', '-', 'i']).nth(0).unwrap();
            let wrapped_val = unparsed_val.trim().parse();
            match wrapped_val {
                Ok(value) => {
                    val = value;
                    string = string.strip_prefix(unparsed_val).unwrap();
                }
                _ => {
                    ok = &false;
                    val = 0.0
                }
            }
        } else {
            val = 1.0;
        }
        if string.starts_with("i") {
            string = string.strip_prefix("i").unwrap();
            if string.is_empty() || !ok {
                Complex {
                    re: 0.0,
                    im: sign * val,
                }
            } else {
                Complex {
                    re: 0.0,
                    im: sign * val,
                } + Complex::from(string)
            }
        } else {
            if string.is_empty() || !ok {
                Complex {
                    re: sign * val,
                    im: 0.0,
                }
            } else {
                Complex {
                    re: sign * val,
                    im: 0.0,
                } + Complex::from(string)
            }
        }
    }
}

//polynomial implementation
struct Polynomial {
    coeficients: Vec<Complex>,
}

impl Default for Polynomial {
    fn default() -> Self {
        Polynomial {
            coeficients: Vec::new(),
        }
    }
}

impl Display for Polynomial {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut polynomial = "".to_string();

        for i in (0..self.coeficients.len()).rev() {
            let val = self.coeficients[i];
            if val != Complex::default() {
                if polynomial.is_empty() {
                    if val.re == 1.0 && val.im == 0.0 {
                        match i {
                            0 => polynomial += format!("1").as_str(),
                            1 => polynomial += format!("X").as_str(),
                            _ => polynomial += format!("X^{}", i).as_str(),
                        }
                    } else if val.re == -1.0 && val.im == 0.0 {
                        match i {
                            0 => polynomial += format!("-1").as_str(),
                            1 => polynomial += format!("-X").as_str(),
                            _ => polynomial += format!("-X^{}", i).as_str(),
                        }
                    } else {
                        match i {
                            0 => polynomial += format!("{}", val).as_str(),
                            1 => polynomial += format!("{}X", val).as_str(),
                            _ => polynomial += format!("{}X^{}", val, i).as_str(),
                        }
                    }
                } else {
                    if val.re == 1.0 && val.im == 0.0 {
                        match i {
                            0 => polynomial += format!(" + 1").as_str(),
                            1 => polynomial += format!(" + X").as_str(),
                            _ => polynomial += format!(" + X^{}", i).as_str(),
                        }
                    } else if val.re == -1.0 && val.im == 0.0 {
                        match i {
                            0 => polynomial += format!(" - 1").as_str(),
                            1 => polynomial += format!(" - X").as_str(),
                            _ => polynomial += format!(" - X^{}", i).as_str(),
                        }
                    } else if val.re < 0.0 || val.re == 0.0 && val.im <= 0.0 {
                        match i {
                            0 => polynomial += format!(" - {}", Complex::default() - val).as_str(),
                            1 => polynomial += format!(" - {}X", Complex::default() - val).as_str(),
                            _ => {
                                polynomial +=
                                    format!(" - {}X^{}", Complex::default() - val, i).as_str()
                            }
                        }
                    } else {
                        match i {
                            0 => polynomial += format!(" + {}", val).as_str(),
                            1 => polynomial += format!(" + {}X", val).as_str(),
                            _ => polynomial += format!(" + {}X^{}", val, i).as_str(),
                        }
                    }
                }
            }
        }

        if polynomial.is_empty() {
            polynomial = "0".to_string();
        }
        f.write_fmt(format_args!("{}", polynomial))
    }
}

//bruh, I'm never touching this again unless it's REALLY broken
impl From<&str> for Polynomial {
    fn from(mut polynomial_string: &str) -> Polynomial {
        let mut polynomial = Polynomial::default();
        let mut sign = 1.0;
        while !polynomial_string.is_empty() {
            polynomial_string = polynomial_string.trim_start();
            if polynomial_string.starts_with("+") {
                sign = 1.0;
                polynomial_string = polynomial_string.strip_prefix("+").unwrap();
            } else if polynomial_string.starts_with("-") {
                sign = -1.0;
                polynomial_string = polynomial_string.strip_prefix("-").unwrap();
            } else if polynomial_string.starts_with("(") {
                polynomial_string = polynomial_string.strip_prefix("(").unwrap();
                let val_string = polynomial_string.split(")").nth(0).unwrap();
                let val = Complex::from(val_string);
                polynomial_string = polynomial_string
                    .strip_prefix(val_string)
                    .unwrap()
                    .strip_prefix(")")
                    .unwrap();
                let mut power: usize = 0;
                if polynomial_string.starts_with(['x', 'X']) {
                    polynomial_string = &polynomial_string.strip_prefix(['x', 'X']).unwrap();
                    power = 1;
                    if polynomial_string.starts_with("^") {
                        polynomial_string = polynomial_string.strip_prefix("^").unwrap();
                        let power_string = polynomial_string.split(['+', '-']).nth(0).unwrap();
                        power = power_string
                            .trim()
                            .parse()
                            .expect("Error while parsing a power");
                        polynomial_string = polynomial_string.strip_prefix(power_string).unwrap();
                    }
                }
                polynomial.add0s(power);
                polynomial.coeficients[power] += val * Complex::from(sign);
            } else if polynomial_string.starts_with(char::is_numeric) {
                let val_string = polynomial_string
                    .split(['x', 'X'])
                    .nth(0)
                    .unwrap()
                    .split(['-', '+'])
                    .nth(0)
                    .unwrap();
                let val = Complex::from(val_string);
                polynomial_string = polynomial_string.strip_prefix(val_string).unwrap();
                let mut power: usize = 0;
                if polynomial_string.starts_with(['x', 'X']) {
                    polynomial_string = polynomial_string.strip_prefix(['x', 'X']).unwrap();
                    power = 1;
                    if polynomial_string.starts_with("^") {
                        polynomial_string = polynomial_string.strip_prefix("^").unwrap();
                        let power_string = polynomial_string.split(['+', '-']).nth(0).unwrap();
                        power = power_string
                            .trim()
                            .parse()
                            .expect("Error while parsing a power");
                        polynomial_string = polynomial_string.strip_prefix(power_string).unwrap();
                    }
                }
                polynomial.add0s(power);
                polynomial.coeficients[power] += val * Complex::from(sign);
            } else if polynomial_string.starts_with(['x', 'X']) {
                polynomial_string = polynomial_string.strip_prefix(['x', 'X']).unwrap();
                let mut power: usize = 1;
                if polynomial_string.starts_with("^") {
                    polynomial_string = polynomial_string.strip_prefix("^").unwrap();
                    let power_string = polynomial_string.split(['+', '-']).nth(0).unwrap();
                    power = power_string
                        .trim()
                        .parse()
                        .expect(format!("Error while parsing a power: {}", power_string).as_str());
                    polynomial_string = polynomial_string.strip_prefix(power_string).unwrap();
                }
                polynomial.add0s(power);
                polynomial.coeficients[power] += Complex::from(sign);
            } else if polynomial_string.starts_with("i") {
                polynomial_string = polynomial_string.strip_prefix("i").unwrap();
                let mut power: usize = 0;
                if polynomial_string.starts_with(['x', 'X']) {
                    polynomial_string = polynomial_string.strip_prefix(['x', 'X']).unwrap();
                    power = 1;
                    if polynomial_string.starts_with("^") {
                        polynomial_string = polynomial_string.strip_prefix("^").unwrap();
                        let power_string = polynomial_string.split(['+', '-']).nth(0).unwrap();
                        power = power_string
                            .trim()
                            .parse()
                            .expect("Error while parsing a power");
                        polynomial_string = polynomial_string.strip_prefix(power_string).unwrap();
                    }
                }
                polynomial.add0s(power);
                polynomial.coeficients[power] += Complex { re: 0.0, im: sign };
            } else {
                return Polynomial::default();
            }
        }
        polynomial
    }
}

impl Polynomial {
    fn value_at(&self, x: Complex) -> Complex {
        let mut factor = Complex { re: 1.0, im: 0.0 };
        let mut solution = Complex::default();

        for i in 0..self.coeficients.len() {
            solution += factor * self.coeficients[i];
            factor = factor * x;
        }
        solution
    }
    fn search_min(&self) -> Complex {
        let mut solution = Complex { re: 0.0, im: 0.0 };
        let mut re = 1e20;
        for _ in 0..150 {
            let mut im = 1e20;
            for _ in 0..150 {
                if self.value_at(solution) > self.value_at(solution + Complex { re, im }) {
                    solution += Complex { re, im };
                } else if self.value_at(solution)
                    > self.value_at(solution + Complex { re: -re, im })
                {
                    solution += Complex { re: -re, im };
                } else if self.value_at(solution)
                    > self.value_at(solution + Complex { re, im: -im })
                {
                    solution += Complex { re, im: -im };
                } else if self.value_at(solution)
                    > self.value_at(solution + Complex { re: -re, im: -im })
                {
                    solution += Complex { re: -re, im: -im };
                }
                if self.value_at(solution) == Complex::default() {
                    break;
                }
                im /= 2.0;
            }
            re /= 2.0;
        }
        solution
    }
    fn horner(&mut self, div: Complex) -> Complex {
        let mut solution: Vec<(usize, Complex)> = vec![];
        for i in (1..self.coeficients.len()).rev() {
            let coeficient: Complex = solution
                .last()
                .unwrap_or(&(0, Complex { re: 0.0, im: 0.0 }))
                .1
                * div
                + self.coeficients[i];
            solution.push((i - 1, coeficient));
        }
        self.coeficients.pop();
        for val in solution {
            self.coeficients[val.0] = val.1;
        }
        Complex { re: 0.0, im: 0.0 }
    }
    fn degree(&self) -> usize {
        let max_power: usize = std::cmp::max(self.coeficients.len(), 1) - 1;
        max_power
    }
    fn trim0s(&mut self) {
        while self
            .coeficients
            .last()
            .unwrap_or(&Complex { re: 1.0, im: 0.0 })
            == &Complex::default()
        {
            self.coeficients.pop();
        }
    }
    fn add0s(&mut self, new_length: usize) {
        let length = self.coeficients.len();
        for _ in length..new_length + 1 {
            self.coeficients.push(Complex::default());
        }
    }
}

fn get_solutions(mut polynomial_string: String) -> Vec<ComplexWithString> {
    polynomial_string = polynomial_string.trim().to_string();

    //parsing hell that's probably still not working well enough -_-
    let mut polynomial = Polynomial::from(polynomial_string.as_str());

    polynomial.trim0s();

    //creating the polynomial output string
    //getting solutions
    let mut solutions: Vec<ComplexWithString> = vec![];
    let max_power: usize = polynomial.degree();
    for _i in 0..max_power {
        let solution = polynomial.search_min();
        solutions.push(ComplexWithString {
            re: solution.re,
            im: solution.im,
            string: format!("{}", solution),
        });
        polynomial.horner(solution);
        polynomial.trim0s();
    }
    solutions
}
