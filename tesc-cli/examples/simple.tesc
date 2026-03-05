test +("python3 calculator.py") {
    self send "1 + 1";
    self expect "2";
};

// Test comment
test -("python3 calculator.py") {
    self send "1 - 2";
    self expect "-1";
};
