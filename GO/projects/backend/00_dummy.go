package main

import (
	"encoding/json"
	"net/http"
)

type Venda struct {
	ID         int     `json:"id"`
	Produto    string  `json:"produto"`
	Quantidade int     `json:"quantidade"`
	Preco      float64 `json:"preco"`
}

var sales []Venda

/*
w: o que vamos devolver para o cliente
r: o que o cliente enviou
*/
func createSale(w http.ResponseWriter, r *http.Request) {
	var v Venda                        // struct vazia
	json.NewDecoder(r.Body).Decode(&v) // aqui populamos v
	sales = append(sales, v)           // apendando na slice

	// escrevemos a resposta para o usuario
	json.NewEncoder(w).Encode(map[string]string{
		"mensagem": "Venda criada",
	})
}

func saleList(w http.ResponseWriter, r *http.Request) {
	json.NewEncoder(w).Encode(sales)
}

func totalSales(w http.ResponseWriter, r *http.Request) {
	total := 0.0
	for _, v := range sales {
		total += float64(v.Quantidade) * v.Preco
	}
	json.NewEncoder(w).Encode(map[string]float64{
		"total": total,
	})
}

func main() {
	http.HandleFunc("/vendas", saleList) // w e r são criadas a partir da requisição do cliente
	http.HandleFunc("/vendas/criar", createSale)
	http.HandleFunc("/total", totalSales)
	http.ListenAndServe(":8080", nil)
}

// rode com: go run main.go e va para a porta 8080
/*
curl -X POST http://localhost:8080/vendas/criar \
-H "Content-Type: application/json" \
-d '{"id":1,"produto":"PC Gamer","quantidade":1,"preco":5700}'
*/
