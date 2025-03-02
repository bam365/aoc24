import { pipe } from "fp-ts/function"
import { Ord as NumOrd } from "fp-ts/number"
import * as RA from "fp-ts/ReadonlyArray"
import { parser as p } from "parser-ts"
import { int as pInt, spaces1 } from "parser-ts/string"
import { char as pChar } from "parser-ts/char"

import { parseAllInputExn, parserDo, arraySum } from "./aoclib.ts"

const sortedPairDiff = (pairs: [number, number][]): number => {
    const sortedXs = pipe(
        pairs,
        RA.map(v => v[0]),
        RA.sort(NumOrd),
    )
    const sortedYs = pipe(
        pairs,
        RA.map(v => v[1]),
        RA.sort(NumOrd),
    )
    const distance = (a: number, b: number) => Math.abs(b - a)
    return pipe(RA.zipWith(sortedXs, sortedYs, distance), arraySum)
}

const numberPair: p.Parser<string, [number, number]> = pipe(
    parserDo,
    p.bind("x", () => pipe(pInt, p.apFirst(spaces1))),
    p.bind("y", () => pInt),
    p.map(d => [d.x, d.y]),
)

const numberPairLines: p.Parser<string, [number, number][]> = p.sepBy(
    pChar("\n"),
    numberPair,
)

export const part1 = async () => {
    const input = await parseAllInputExn(numberPairLines)
    console.log(sortedPairDiff(input))
}

part1()
