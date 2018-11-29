class HelloWorld {
    public static hello(message?: string) {
        return `Hello, ${message ? message : 'World'}!`
    }
}

export default HelloWorld