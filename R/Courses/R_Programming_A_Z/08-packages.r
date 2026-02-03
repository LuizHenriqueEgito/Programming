library(ggplot2)

ggplot(
  data = diamonds,
  aes(
    x = carat,
    y = price,
    colour = clarity
  )
) +
  geom_point() +
  facet_wrap(~ clarity)
