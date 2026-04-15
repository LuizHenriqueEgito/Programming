# Importando o dplyr

``` r
library(dplyr)
```

    ## 
    ## Anexando pacote: 'dplyr'

    ## Os seguintes objetos são mascarados por 'package:stats':
    ## 
    ##     filter, lag

    ## Os seguintes objetos são mascarados por 'package:base':
    ## 
    ##     intersect, setdiff, setequal, union

``` r
library(readr)

URL = "https://raw.githubusercontent.com/datasciencedojo/datasets/master/titanic.csv"
df <- read_csv(URL)
```

    ## Rows: 891 Columns: 12

    ## ── Column specification ────────────────────────────────────────────────────────
    ## Delimiter: ","
    ## chr (5): Name, Sex, Ticket, Cabin, Embarked
    ## dbl (7): PassengerId, Survived, Pclass, Age, SibSp, Parch, Fare
    ## 
    ## ℹ Use `spec()` to retrieve the full column specification for this data.
    ## ℹ Specify the column types or set `show_col_types = FALSE` to quiet this message.

``` r
df
```

    ## # A tibble: 891 × 12
    ##    PassengerId Survived Pclass Name   Sex     Age SibSp Parch Ticket  Fare Cabin
    ##          <dbl>    <dbl>  <dbl> <chr>  <chr> <dbl> <dbl> <dbl> <chr>  <dbl> <chr>
    ##  1           1        0      3 Braun… male     22     1     0 A/5 2…  7.25 <NA> 
    ##  2           2        1      1 Cumin… fema…    38     1     0 PC 17… 71.3  C85  
    ##  3           3        1      3 Heikk… fema…    26     0     0 STON/…  7.92 <NA> 
    ##  4           4        1      1 Futre… fema…    35     1     0 113803 53.1  C123 
    ##  5           5        0      3 Allen… male     35     0     0 373450  8.05 <NA> 
    ##  6           6        0      3 Moran… male     NA     0     0 330877  8.46 <NA> 
    ##  7           7        0      1 McCar… male     54     0     0 17463  51.9  E46  
    ##  8           8        0      3 Palss… male      2     3     1 349909 21.1  <NA> 
    ##  9           9        1      3 Johns… fema…    27     0     2 347742 11.1  <NA> 
    ## 10          10        1      2 Nasse… fema…    14     1     0 237736 30.1  <NA> 
    ## # ℹ 881 more rows
    ## # ℹ 1 more variable: Embarked <chr>

# Olhando as primeiras 10 linhas (HEAD)

``` r
df %>%
  head(5)
```

    ## # A tibble: 5 × 12
    ##   PassengerId Survived Pclass Name    Sex     Age SibSp Parch Ticket  Fare Cabin
    ##         <dbl>    <dbl>  <dbl> <chr>   <chr> <dbl> <dbl> <dbl> <chr>  <dbl> <chr>
    ## 1           1        0      3 Braund… male     22     1     0 A/5 2…  7.25 <NA> 
    ## 2           2        1      1 Cuming… fema…    38     1     0 PC 17… 71.3  C85  
    ## 3           3        1      3 Heikki… fema…    26     0     0 STON/…  7.92 <NA> 
    ## 4           4        1      1 Futrel… fema…    35     1     0 113803 53.1  C123 
    ## 5           5        0      3 Allen,… male     35     0     0 373450  8.05 <NA> 
    ## # ℹ 1 more variable: Embarked <chr>

# Fazendo filtros

``` r
df %>%
  # filtra
  filter(Survived == 1) %>% 
  # faz o select de algumas colunas
  select(Name, Age, Sex, Survived)
```

    ## # A tibble: 342 × 4
    ##    Name                                                  Age Sex    Survived
    ##    <chr>                                               <dbl> <chr>     <dbl>
    ##  1 Cumings, Mrs. John Bradley (Florence Briggs Thayer)    38 female        1
    ##  2 Heikkinen, Miss. Laina                                 26 female        1
    ##  3 Futrelle, Mrs. Jacques Heath (Lily May Peel)           35 female        1
    ##  4 Johnson, Mrs. Oscar W (Elisabeth Vilhelmina Berg)      27 female        1
    ##  5 Nasser, Mrs. Nicholas (Adele Achem)                    14 female        1
    ##  6 Sandstrom, Miss. Marguerite Rut                         4 female        1
    ##  7 Bonnell, Miss. Elizabeth                               58 female        1
    ##  8 Hewlett, Mrs. (Mary D Kingcome)                        55 female        1
    ##  9 Williams, Mr. Charles Eugene                           NA male          1
    ## 10 Masselmani, Mrs. Fatima                                NA female        1
    ## # ℹ 332 more rows

# Adicionando nova coluna (mutate)

``` r
df %>% 
  mutate(
    N_Family = SibSp + Parch,
    Times_Family = SibSp * Parch,
    Dummy_Sex = ifelse(Sex == "male", 1, 0),
    Pow_Age = Age ^ 2,
    Cat_Age = case_when(
      Sex == "female" & Age < 12 ~ "Prioridade_Maxima",
      (Sex == "female" & Age >= 12) | (Sex == "male" & Age < 12) ~ "Prioridade_Alta",
      Sex == "male" & Age < 12 ~ "Prioridade_Media",
      Sex == "male" & Age >= 12 ~ "Prioridade_Baixa",
      TRUE ~ "Sem_prioridade"
    )
    
  ) %>% 
  filter(Times_Family > N_Family) %>% 
  select(
    Name,
    Age,
    SibSp,
    Parch,
    N_Family,
    Times_Family,
    Dummy_Sex,
    Pow_Age,
    Cat_Age
  )
```

    ## # A tibble: 29 × 9
    ##    Name          Age SibSp Parch N_Family Times_Family Dummy_Sex Pow_Age Cat_Age
    ##    <chr>       <dbl> <dbl> <dbl>    <dbl>        <dbl>     <dbl>   <dbl> <chr>  
    ##  1 Fortune, M…    19     3     2        5            6         1     361 Priori…
    ##  2 Goodwin, M…    11     5     2        7           10         1     121 Priori…
    ##  3 Skoog, Mas…     4     3     2        5            6         1      16 Priori…
    ##  4 Andersson,…    17     4     2        6            8         0     289 Priori…
    ##  5 Goodwin, M…    16     5     2        7           10         0     256 Priori…
    ##  6 Fortune, M…    23     3     2        5            6         0     529 Priori…
    ##  7 Andersson,…     2     4     2        6            8         0       4 Priori…
    ##  8 Sage, Mast…    NA     8     2       10           16         1      NA Sem_pr…
    ##  9 Sage, Miss…    NA     8     2       10           16         0      NA Sem_pr…
    ## 10 Asplund, M…     9     4     2        6            8         1      81 Priori…
    ## # ℹ 19 more rows
