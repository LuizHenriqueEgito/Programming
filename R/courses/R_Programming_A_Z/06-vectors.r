# VECTORS
my_vector <- c(3, 45, 56, 732)
print(my_vector)
my_vector
is.numeric(my_vector)
is.integer(my_vector)
is.double(my_vector)
my_vector_int <- c(3L, 45L, 56L, 732L)
is.integer(my_vector_int)

my_string_vector = c("a", "b", "c")
is.character(my_string_vector)
is.numeric(my_string_vector)
seq(1, 15)
seq(1, 15) == c(1: 15)
z <- seq(1, 15, 4)
z
rep(3, 10)

x <- c(1, 123, 534, 13, 4)  # combine
y <- seq(201, 250, 11)  # sequence
z <- rep("Hi!", 3)  # replicate

# remove the index
x[-4]
# pega 123, o indice no R começa no 1 não no 0
x[2]
x[1:3]
# VECTOR ARITHMETRICS
v <- c(50, 34, 111, 7, 24, 631, 20, 4, 7, 21)
w <- c(100, 2, 56, 12, 0, 65, 93, 10, 244, 1)

v + w
v - w
v * w
v / w