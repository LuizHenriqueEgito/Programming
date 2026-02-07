# ggplot2 é composto por 7 camadas, são elas:
# 1. Data
# 2. Aesthetics
# 3. Geometries
# 4. Statistics
# 5. Facets
# 6. Coordinates
# 7. Theme
library(ggplot2)

head(mpg)
getwd()

# Primeiro, crie o gráfico
ggplot(
        data = mpg, 
        aes(x = displ, y = hwy, colour = manufacturer, size = cty)
    ) + 
    # add geometry
    geom_point() +
    # add statistics
    geom_smooth(fill=NA) +
    facet_grid(year~.)

# Salva o último gráfico exibido
PATH <- "/home/smith/Documentos/Devops/Programming/R/Courses/R_Programming_A_Z"
ggsave(
    "11-graph.jpg",
    path = PATH
)