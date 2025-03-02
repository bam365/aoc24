import { pipe } from "fp-ts/function"
import * as E from "fp-ts/Either"
import * as RA from "fp-ts/ReadonlyArray"
import { parser as p } from "parser-ts"
import { run } from "parser-ts/code-frame"

export const readAllStdin = (): Promise<string> => Bun.stdin.text()

export const parseAllInputExn = async <T>(
    parser: p.Parser<string, T>,
): Promise<T> => {
    const input = await readAllStdin()
    return pipe(
        run(parser, input),
        E.match(
            () => {
                throw new Error("Can't parse input")
            },
            v => v,
        ),
    )
}

export const parserDo: p.Parser<string, {}> = p.of({})

export const arraySum = (xs: ReadonlyArray<number>): number =>
    pipe(
        xs,
        RA.reduce(0, (x, y) => x + y),
    )
