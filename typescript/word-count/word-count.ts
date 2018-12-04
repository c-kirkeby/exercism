
export default class Words {
    public count(words: string): Map<string, number> | undefined {
        const tokens = words.toLowerCase()
                            .match(/\S+/g) // Tokenise by whitespace

        if (tokens && tokens.length > 0) {
            return tokens.reduce((accumulator, item, _index, _source) => {
                if (accumulator.has(item)) {
                    // Check if the Map already has a record with the key and if so, increment it
                    return accumulator.set(item, accumulator.get(item) + 1)
                } else {
                    // Otherwise create a new entry in the Map with an initial count
                    return accumulator.set(item, 1)
                }
            }, new Map())
        }
        return undefined
    }
}