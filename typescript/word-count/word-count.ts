
export default class Words {
    public count(words: string): Map<string, number> | undefined {
        const tokens = words.toLowerCase()
                            .match(/\S+/g) // Tokenise by whitespace
        // Check if result returned by match() is not null
        if (tokens) {
            return tokens.reduce((accumulator, item) => {
                return accumulator.set(item, accumulator.get(item) + 1 || 1)
            }, new Map())
        }
        return undefined
    }
}