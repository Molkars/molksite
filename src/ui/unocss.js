
window.__unocss = {
    rules: [
        [/^m-\d+$/, groups => ({
            'margin': `${groups[1]}px`
        })],
        [/^m-\d+-\d+$/, groups => ({
            'margin': `${groups[1]}px ${groups[2]}px`
        })],
        [/^p-\d+$/, groups => ({
            'padding': `${groups[1]}px`
        })],
        [/^p-\d+-\d+$/, groups => ({
            'padding': `${groups[1]}px ${groups[2]}px`
        })],
    ],
    presents: [

    ],
}