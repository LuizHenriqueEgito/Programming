cat("Using matrix()\n")
my_data <- 1:20
n_rows <- 4
n_cols <- 5
A <- matrix(my_data, n_rows, n_cols)
print(A)
#     [,1] [,2] [,3] [,4] [,5]
# [1,]    1    5    9   13   17
# [2,]    2    6   10   14   18
# [3,]    3    7   11   15   19
# [4,]    4    8   12   16   20
cat("\nUsing rbind()\n")
row_1 <- c("I", "am", "happy")
row_2 <- c("what", "a", "day")
row_3 <- c(1, 2, 3)
B <- rbind(row_1, row_2, row_3)
print(B)
#       [,1]   [,2] [,3]   
# row_1 "I"    "am" "happy"
# row_2 "what" "a"  "day"  
# row_3 "1"    "2"  "3"  

cat("\nUsing cbind()\n")
col_1 <- c(1:3) 
col_2 <- c(4:6)
col_3 <- c(7:9)
C <- cbind(col_1, col_2, col_3)
print(C)
#      col_1 col_2 col_3
# [1,]     1     4     7
# [2,]     2     5     8
# [3,]     3     6     9
cat("\n\n")
# Matriz operations
A <- matrix(c(1:9), 3, 3)
print(A)

B <- matrix(c(11:19), 3, 3)
print(B)
C <- round(A / B, 2)
print(C)
D <- A - B
E <- A + B
F <- A * B
G <- A %*% B  # matmul
print(D)
print(E)
print(F)
print(G)