// Modified version of https://github.com/codemirror/theme-one-dark/blob/b2783a648d8d94e544993c7c1467dfea6ec86618/src/one-dark.ts

/// The editor theme styles for One Dark.
import { HighlightStyle, syntaxHighlighting } from '@codemirror/language'
import { EditorView } from '@codemirror/view'
import { tags as t } from '@lezer/highlight'

const highlightBackground = '#2c313a',
    tooltipBackground = '#353a42',
    //////////// Below colors taken from https://github.com/navarasu/onedark.nvim/blob/fdfe7bfff486acd102aae7fb2ff52e7e5f6c2bad/lua/onedark/palette.lua
    bg0 = '#282c34',
    bg3 = '#3b3f4c',
    bg_d = '#21252b',
    fg = '#abb2bf',
    purple = '#c678dd',
    green = '#98c379',
    orange = '#d19a66',
    yellow = '#e1bd79',
    blue = '#61afef',
    cyan = '#56b6c2',
    red = '#e86671',
    grey = '#5c6370',
    light_grey = '#848b98',
    rainbow_gradient = "#ffffff",
    hpi_gradient = "#ffff00"

export const oneDarkTheme = EditorView.theme({
    '.ͼv': {
        background: "linear-gradient(90deg, rgba(255, 0, 0, 1) 0%, rgba(255, 154, 0, 1) 10%, rgba(208, 222, 33, 1) 20%, rgba(79, 220, 74, 1) 30%, rgba(63, 218, 216, 1) 40%, rgba(47, 201, 226, 1) 50%, rgba(28, 127, 238, 1) 60%, rgba(95, 21, 242, 1) 70%, rgba(186, 12, 248, 1) 80%, rgba(251, 7, 217, 1) 90%, rgba(255, 0, 0, 1) 100%)",
        '-webkit-background-clip': 'text',
        '-webkit-text-fill-color': "transparent",
        'filter': "saturate(80%)",
    },
    '.ͼt': {
        background: 'linear-gradient(90deg, rgba(176,12,54,1) 0%, rgba(226,86,23,1) 50%, rgba(248,156,14,1) 100%)',
        '-webkit-background-clip': 'text',
        '-webkit-text-fill-color': "transparent",
        'filter': "saturate(80%)",
    },
    '*': {
        fontFamily: 'JetBrains Mono NL, monospace',
    },
    '&': {
        color: fg,
        backgroundColor: bg0,
        height: '100%',
    },
    '.cm-scroller': { overflow: 'auto' },

    '.cm-content': {
        caretColor: blue,
    },

    '.cm-cursor, .cm-dropCursor': { borderLeftColor: blue },
    '&.cm-focused .cm-selectionBackground, .cm-selectionBackground, .cm-content ::selection': {
        backgroundColor: bg3,
    },

    '.cm-panels': { backgroundColor: bg_d, color: fg },
    '.cm-panels.cm-panels-top': { borderBottom: '2px solid black' },
    '.cm-panels.cm-panels-bottom': { borderTop: '2px solid black' },

    '.cm-searchMatch': {
        backgroundColor: '#72a1ff59',
        outline: '1px solid #457dff',
    },
    '.cm-searchMatch.cm-searchMatch-selected': {
        backgroundColor: '#6199ff2f',
    },

    '.cm-activeLine': { backgroundColor: highlightBackground },
    '.cm-selectionMatch': { backgroundColor: '#aafe661a' },

    '&.cm-focused .cm-matchingBracket, &.cm-focused .cm-nonmatchingBracket': {
        backgroundColor: '#bad0f847',
        outline: '1px solid #515a6b',
    },

    '.cm-gutters': {
        backgroundColor: bg0,
        color: grey,
        border: 'none',
    },

    '.cm-activeLineGutter': {
        backgroundColor: highlightBackground,
    },

    '.cm-foldPlaceholder': {
        backgroundColor: 'transparent',
        border: 'none',
        color: '#ddd',
    },

    '.cm-tooltip': {
        border: 'none',
        backgroundColor: tooltipBackground,
    },
    '.cm-tooltip .cm-tooltip-arrow:before': {
        borderTopColor: 'transparent',
        borderBottomColor: 'transparent',
    },
    '.cm-tooltip .cm-tooltip-arrow:after': {
        borderTopColor: tooltipBackground,
        borderBottomColor: tooltipBackground,
    },
    '.cm-tooltip-autocomplete': {
        '& > ul > li[aria-selected]': {
            backgroundColor: highlightBackground,
            color: fg,
        },
    },
}, { dark: true })

/// The highlighting style for code in the One Dark theme.
export const oneDarkHighlightStyle = HighlightStyle.define([
    { tag: t.keyword, color: purple },
    { tag: [t.variableName], color: fg},
    { tag: [ t.operator], color: fg },
    { tag: [t.bool, t.null, t.typeName, t.number], color: orange },
    {tag: [t.namespace], color: hpi_gradient},
    { tag: [t.typeOperator], color: yellow },
    { tag: [t.labelName], color: rainbow_gradient },
    { tag: [t.function(t.propertyName), t.function(t.variableName)], color: blue },
    { tag: [t.propertyName, t.standard(t.function(t.variableName))], color: cyan },
    { tag: [t.local(t.variableName), t.standard(t.variableName), t.className], color: red },
    { tag: t.comment, color: grey },
    { tag: t.character, color: orange },
    { tag: [t.bracket, t.separator], color: light_grey },
    { tag: t.string, color: green },
])

/// Extension to enable the One Dark theme (both the editor theme and
/// the highlight style).
export const oneDark = [oneDarkTheme, syntaxHighlighting(oneDarkHighlightStyle)]
