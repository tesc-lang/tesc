test add("python3 calculator.py") {
    input("1 + 1");
    output("2");
}

test sub("python3 calculator.py") {
    input("1 - 2");
    output("-1");
}

test fail("python3 calculator.py") {
    input("1");
    output("-1");
}
