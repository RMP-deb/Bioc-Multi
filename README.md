# Bioc-Multi: a Bioconductor package template with optional C++ and Rust

A starting point for an R package aimed at [Bioconductor](https://bioconductor.org/),
following the [Bioconductor package guidelines](https://contributions.bioconductor.org/).
Every package gets R code. You can also add compiled code in **C++** (through
[Rcpp](https://www.rcpp.org/)), in **Rust** (through [extendr](https://extendr.github.io/)),
or both. One setup command removes the parts you don't need and renames everything to
your package name.

> **This README describes the template, not your package.** `setup.R` deletes it. Your
> package's own README is generated from `README.Rmd`, which you edit later.
> For a CRAN package, use [CRAN-Multi](https://github.com/RMP-deb/CRAN-Multi) instead.

## How the template works

The repository is a complete, working package called `pkgname`, with one small
example function in each language. Names are camelCase, following Bioconductor style:

| Language | Example function | Source | Test |
|---|---|---|---|
| R | `hello()` | `R/hello.R` | `tests/testthat/test-hello.R` |
| C++ | `sumSquaresCpp()` | `src/sumSquaresCpp.cpp` | `tests/testthat/test-sumSquaresCpp.R` |
| Rust | `sumSquaresRust()` | `src/rust/src/lib.rs` | `tests/testthat/test-sumSquaresRust.R` |

When you run `Rscript setup.R <name> [--cpp] [--rust]`, the script:

1. **Removes the languages you didn't pick.** It deletes their source files, tests,
   man pages and build files, and removes their entries from `DESCRIPTION`,
   `NAMESPACE`, `README.Rmd`, the vignette, the `.gitignore`/`.Rbuildignore` files and
   the GitHub Actions workflow.
2. **Renames the package.** It replaces `pkgname` everywhere, in file contents and in
   file names (e.g. `pkgname.Rproj` becomes `mynewpkg.Rproj`).
3. **Regenerates the C++ glue code** if you kept C++ (`Rcpp::compileAttributes()`).
4. **Deletes itself and this README.**

What's left is a package that passes `R CMD check` and `BiocCheck` (0 errors,
0 warnings) with only the languages you asked for. All four combinations (R, R + C++,
R + Rust, R + C++ + Rust) are tested.

Compared with a CRAN package, the template already has: version `0.99.0`, a
`biocViews` field, a BiocStyle vignette, `BiocManager` install instructions, 4-space
indentation, camelCase names, `inst/scripts/` for data provenance, and a CI job that
runs `BiocCheck` on Bioconductor devel.

## Step by step

### 1. Install what you need

| You want | Install |
|---|---|
| Any package | R (>= 4.6), plus `install.packages(c("devtools", "roxygen2", "testthat", "knitr", "rmarkdown", "BiocManager"))` and `BiocManager::install(c("BiocCheck", "BiocStyle"))` |
| C++ | A C++ compiler (Rtools on Windows, Xcode command line tools on macOS, `build-essential` on Linux) and `install.packages("Rcpp")` |
| Rust | The Rust toolchain from <https://rustup.rs> and `install.packages("rextendr")`. **Read "Rust on Bioconductor" below first** |
| Vignettes | [pandoc](https://pandoc.org/installing.html). RStudio includes a copy |

### 2. Pick a name

Bioconductor requires a name that isn't used on CRAN or Bioconductor:
`pak::pkg_name_check("mynewpkg")`. Use letters, numbers and `.`. If you'll use Rust,
use letters and numbers only: the name is also the Rust crate name, and crate names
can't contain `.`.

### 3. Create your own copy

Pick one of these. The first two give you a new GitHub repository with a clean history.

**With the GitHub CLI**

```sh
gh repo create mynewpkg --public --template RMP-deb/Bioc-Multi --clone
cd mynewpkg
```

(Bioconductor reviews public GitHub repositories, so `--public` is the usual choice.)

**On the GitHub website**: click **Use this template → Create a new repository**, then
clone the new repository and `cd` into it.

**Without GitHub**

```sh
cp -r Bioc-Multi ~/code/mynewpkg
cd ~/code/mynewpkg
rm -rf .git          # drop the template's own history
git init
```

### 4. Run the setup script (once)

From inside your copy, run **one** of these:

```sh
Rscript setup.R mynewpkg                 # R only
Rscript setup.R mynewpkg --cpp           # R + C++
Rscript setup.R mynewpkg --rust          # R + Rust
Rscript setup.R mynewpkg --cpp --rust    # R + C++ + Rust
```

It prints something like `Set up 'mynewpkg' with R + C++. Next: see SETUP.md`.

### 5. Fill in the placeholders

| File | What to change |
|---|---|
| `DESCRIPTION` | `Title`; `Description` (at least 3 sentences); `Authors@R` (your name, email, and ORCID if you have one); `USERNAME` in `URL` and `BugReports` |
| `DESCRIPTION` | `biocViews`: replace the placeholder `Software, Infrastructure` with terms from the [biocViews list](https://bioconductor.org/packages/devel/BiocViews.html#___Software) |
| `README.Rmd` | What the package does and an example. Render it with `devtools::build_readme()` |
| `vignettes/mynewpkg.Rmd` | Author, abstract and a real worked example. Bioconductor requires a vignette |
| `_pkgdown.yml` | `USERNAME`, or delete the file if you don't want a pkgdown site |
| `inst/scripts/make-data.R` | How your data was made, or delete it if you ship no data |

### 6. Build and check it

Open the project in RStudio (`mynewpkg.Rproj`) or any R session in the folder:

```r
devtools::document()     # generates NAMESPACE and man/ (and compiles C++/Rust code)
devtools::test()         # runs the example tests
devtools::check()        # full R CMD check
BiocCheck::BiocCheck()   # Bioconductor's extra checks
```

The first build with Rust downloads and compiles the Rust dependencies, which takes a
minute or two. Later builds are faster.

### 7. Replace the examples with your code

Delete the example functions and their tests, and write your own:

* **R**: `usethis::use_r("myFun")` and `usethis::use_test("myFun")`.
* **C++**: add `.cpp` files to `src/` and put `// [[Rcpp::export]]` above each function
  R should see.
* **Rust**: add functions to `src/rust/src/lib.rs` with
  `#[extendr(r_name = "myFun")]` and list them in the `extendr_module!` block.

Document everything with roxygen comments (`#'` in R, `//'` in C++, `///` in Rust),
then run `devtools::document()` again. Use Bioconductor classes such as
`SummarizedExperiment` or `GRanges` for inputs and outputs where they fit: reviewers
expect it, and it clears BiocCheck's "no Bioconductor dependencies" message.

### 8. Commit and push

```sh
git add -A
git commit -m "Set up mynewpkg from template"
git push
```

GitHub Actions then runs `R CMD check` and `BiocCheck` on Bioconductor devel, plus
`R CMD check` on macOS and Windows, for every push and pull request
(`.github/workflows/R-CMD-check.yaml`).

### 9. Submit to Bioconductor

When `R CMD check` and `BiocCheck` are clean, follow SETUP.md section 9: register on the
Bioconductor support site, join the bioc-devel mailing list, and open an issue at
<https://github.com/Bioconductor/Contributions>. Bump the version (`0.99.1`,
`0.99.2`, ...) with every change you push during review.

`SETUP.md` is the detailed reference: the full file layout, style rules, versioning, C++
and Rust details, and the submission steps. It's excluded from the package build, so
you can keep it while you work and delete it later.

## Things to know

* **Rust on Bioconductor**: Bioconductor hasn't formally adopted Rust, and not all of its
  build machines have it installed. Ask on the
  [bioc-devel mailing list](https://stat.ethz.ch/mailman/listinfo/bioc-devel) before
  building a package around Rust. C++ is the safe choice.
* **C++ and Rust together**: when you add, remove or rename a Rust function, run
  `devtools::document()`, then `Rcpp::compileAttributes()`, then `devtools::document()`
  again. A single `document()` isn't enough, because Rcpp has to register the Rust
  functions too. Changes inside existing functions need just one `document()`. See
  SETUP.md section 7.
* **Adding a language later** isn't automated. Copy the matching files from a fresh copy
  of this template and adjust `DESCRIPTION` (SETUP.md lists which files belong to which
  language).
* **Folder paths with spaces** work, including for Rust builds.
