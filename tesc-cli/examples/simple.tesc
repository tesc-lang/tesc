test +("python3 calculator.py") {
    self send "1 + 1" expect "2" send "1 + 2" expect "3";
};

// Test comment
test -("python3 calculator.py") {
    self send "1 - 2";
    self expect "-1";
    self send "1 - 2";
    self expect "-1";
    self expect "-1";
};
