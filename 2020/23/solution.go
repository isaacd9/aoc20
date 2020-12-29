package main

import (
	"container/ring"
	"fmt"
	"log"
	"sort"
	"strings"
)

type Game struct {
	current       *ring.Ring
	cupIndex      map[uint64]*ring.Ring
	sortedCupList []uint64

	min uint64
}

func (g *Game) String() string {
	var v = []string{}
	g.current.Do(func(i interface{}) {
		v = append(v, fmt.Sprintf("%+v", i))
	})

	joined := strings.Join(v, "  ")
	return joined
}

func (g *Game) Index() string {
	var st = []string{}
	for k, v := range g.cupIndex {
		st = append(st, fmt.Sprintf("%d=%d", k, v.Value))
	}

	joined := strings.Join(st, "  ")
	return joined
}

func (g *Game) findDestination(desired uint64, selected ...uint64) *ring.Ring {
	for desired > g.min {
		desired--

		el := g.cupIndex[desired]
		if el != nil {
			return el
		}
	}

	//log.Printf("sortedCupList: %+v", g.sortedCupList)
	for _, item := range g.sortedCupList {
		el := g.cupIndex[item]
		if el != nil {
			return el
		}
	}

	return nil
}

func (g *Game) PlayRound() {
	cur := g.current.Value.(uint64)

	// Remove cups
	first := g.current.Unlink(1).Value.(uint64)
	second := g.current.Unlink(1).Value.(uint64)
	third := g.current.Unlink(1).Value.(uint64)
	delete(g.cupIndex, first)
	delete(g.cupIndex, second)
	delete(g.cupIndex, third)

	//log.Printf("Pick up %d, %d, %d", first, second, third)

	// Find destination
	dest := g.findDestination(cur, first, second, third)
	//log.Printf("Destination %d", dest.Value.(uint64))

	// Reinsert cups
	g.cupIndex[first] = &ring.Ring{Value: first}
	g.cupIndex[second] = &ring.Ring{Value: second}
	g.cupIndex[third] = &ring.Ring{Value: third}
	dest.Link(g.cupIndex[third])
	dest.Link(g.cupIndex[second])
	dest.Link(g.cupIndex[first])
}

func (g *Game) Play(rounds int) {
	for i := 0; i < rounds; i++ {
		if i%1000000 == 0 {
			log.Printf("--- Move %d ---", i)
		}
		//log.Printf("cups: %s", g)
		//log.Printf("index: %s", g.Index())
		g.PlayRound()
		g.current = g.current.Next()
	}
}

func (g *Game) After(ii uint64) string {
	r := g.cupIndex[ii]
	var v []string
	r.Next().Do(func(i interface{}) {
		if i == ii {
			return
		}
		v = append(v, fmt.Sprintf("%+v", i))
	})

	return strings.Join(v, "")
}

func (g *Game) Next(ii uint64) uint64 {
	r := g.cupIndex[ii]
	return r.Next().Value.(uint64)
}

func NewGame(cups []uint64) *Game {
	cupIndex := map[uint64]*ring.Ring{}
	li := ring.New(len(cups))
	first := li
	min := cups[0]
	for _, c := range cups {
		li.Value = c
		cupIndex[c] = li
		li = li.Next()

		if c < min {
			min = c
		}
	}

	sortedCupList := make([]uint64, 0, len(cups))
	for _, cup := range cups {
		sortedCupList = append(sortedCupList, cup)
	}

	sort.SliceStable(sortedCupList, func(i, j int) bool {
		return sortedCupList[i] > sortedCupList[j]
	})

	return &Game{
		current:       first,
		min:           min,
		sortedCupList: sortedCupList,
		cupIndex:      cupIndex,
	}
}

func fill(arr *[]uint64, upto uint64) {
	var max uint64
	for _, a := range *arr {
		if a > max {
			max = a
		}
	}

	for i := max + 1; i <= upto; i++ {
		*arr = append(*arr, i)
	}
}

func main() {
	//g := NewGame([]uint64{3, 8, 9, 1, 2, 5, 4, 6, 7})
	//g.Play(10)
	//fmt.Printf("%+v\n", g.After(1))

	cups := []uint64{3, 8, 9, 1, 2, 5, 4, 6, 7}
	fill(&cups, 1000000)
	g := NewGame(cups)
	g.Play(10000000)
	fmt.Printf("%+v %+v\n", g.Next(1), g.Next(g.Next(1)))

	//cups := []uint64{1, 3, 5, 4, 6, 8, 7, 2, 9}
	//fill(&cups, 1000000)
	//g := NewGame([]uint64{1, 3, 5, 4, 6, 8, 7, 2, 9})
	//g.Play(10000000)
	//fmt.Printf("%+v\n", g.After(1))
}
