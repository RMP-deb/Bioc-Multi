test_that("sumSquaresCpp() matches R", {
    expect_equal(sumSquaresCpp(c(1, 2, 3)), 14)
    expect_equal(sumSquaresCpp(numeric()), 0)
    x <- runif(100)
    expect_equal(sumSquaresCpp(x), sum(x^2))
})
