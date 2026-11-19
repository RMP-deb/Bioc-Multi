test_that("sumSquaresRust() matches R", {
    expect_equal(sumSquaresRust(c(1, 2, 3)), 14)
    expect_equal(sumSquaresRust(numeric()), 0)
    x <- runif(100)
    expect_equal(sumSquaresRust(x), sum(x^2))
})
