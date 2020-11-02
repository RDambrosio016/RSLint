//! Global variables avaliable within the JS environment

/// Globals provided by Node.js
pub static NODE_GLOBALS: &[&str] = &[
    "Buffer",
    "__dirname",
    "__filename",
    "clearImmediate",
    "clearInterval",
    "clearTimeout",
    "console",
    "exports",
    "global",
    "module",
    "process",
    "queueMicrotask",
    "require",
    "setImmediate",
    "setInterval",
    "setTimeout",
    "TextDecoder",
    "TextEncoder",
    "URL",
    "URLSearchParams",
    "WebAssembly",
];

/// Globals provided by ECMA JavaScript
pub static ECMA_GLOBALS: &[&str] = &[
    "Array",
    "Boolean",
    "Date",
    "decodeURI",
    "decodeURIComponent",
    "encodeURI",
    "encodeURIComponent",
    "Error",
    "eval",
    "EvalError",
    "Function",
    "hasOwnProperty",
    "isFinite",
    "isNaN",
    "JSON",
    "Math",
    "Map",
    "Number",
    "Object",
    "parseInt",
    "parseFloat",
    "RangeError",
    "ReferenceError",
    "RegExp",
    "Set",
    "String",
    "SyntaxError",
    "TypeError",
    "URIError",
    "WeakMap",
];

/// Globals provided by the browser
pub static BROWSER_GLOBALS: &[&str] = &[
    "ArrayBuffer",
    "ArrayBufferView",
    "Audio",
    "Blob",
    "addEventListener",
    "applicationCache",
    "atob",
    "blur",
    "btoa",
    "clearInterval",
    "clearTimeout",
    "close",
    "closed",
    "CustomEvent",
    "DataView",
    "DOMParser",
    "defaultStatus",
    "document",
    "Element",
    "ElementTimeControl",
    "event",
    "FileReader",
    "Float32Array",
    "Float64Array",
    "FormData",
    "focus",
    "frames",
    "getComputedStyle",
    "HTMLElement",
    "HTMLAnchorElement",
    "HTMLBaseElement",
    "HTMLBlockquoteElement",
    "HTMLBodyElement",
    "HTMLBRElement",
    "HTMLButtonElement",
    "HTMLCanvasElement",
    "HTMLDirectoryElement",
    "HTMLDivElement",
    "HTMLDListElement",
    "HTMLFieldSetElement",
    "HTMLFontElement",
    "HTMLFormElement",
    "HTMLFrameElement",
    "HTMLFrameSetElement",
    "HTMLHeadElement",
    "HTMLHeadingElement",
    "HTMLHRElement",
    "HTMLHtmlElement",
    "HTMLIFrameElement",
    "HTMLImageElement",
    "HTMLInputElement",
    "HTMLIsIndexElement",
    "HTMLLabelElement",
    "HTMLLayerElement",
    "HTMLLegendElement",
    "HTMLLIElement",
    "HTMLLinkElement",
    "HTMLMapElement",
    "HTMLMenuElement",
    "HTMLMetaElement",
    "HTMLModElement",
    "HTMLObjectElement",
    "HTMLOListElement",
    "HTMLOptGroupElement",
    "HTMLOptionElement",
    "HTMLParagraphElement",
    "HTMLParamElement",
    "HTMLPreElement",
    "HTMLQuoteElement",
    "HTMLScriptElement",
    "HTMLSelectElement",
    "HTMLStyleElement",
    "HTMLTableCaptionElement",
    "HTMLTableCellElement",
    "HTMLTableColElement",
    "HTMLTableElement",
    "HTMLTableRowElement",
    "HTMLTableSectionElement",
    "HTMLTextAreaElement",
    "HTMLTitleElement",
    "HTMLUListElement",
    "HTMLVideoElement",
    "history",
    "Int16Array",
    "Int32Array",
    "Int8Array",
    "Image",
    "length",
    "localStorage",
    "location",
    "MessageChannel",
    "MessageEvent",
    "MessagePort",
    "MouseEvent",
    "moveBy",
    "moveTo",
    "MutationObserver",
    "name",
    "Node",
    "NodeFilter",
    "navigator",
    "onbeforeunload",
    "onblur",
    "onerror",
    "onfocus",
    "onload",
    "onresize",
    "onunload",
    "open",
    "openDatabase",
    "opener",
    "Option",
    "parent",
    "print",
    "removeEventListener",
    "resizeBy",
    "resizeTo",
    "screen",
    "scroll",
    "scrollBy",
    "scrollTo",
    "sessionStorage",
    "setInterval",
    "setTimeout",
    "SharedWorker",
    "status",
    "SVGAElement",
    "SVGAltGlyphDefElement",
    "SVGAltGlyphElement",
    "SVGAltGlyphItemElement",
    "SVGAngle",
    "SVGAnimateColorElement",
    "SVGAnimateElement",
    "SVGAnimateMotionElement",
    "SVGAnimateTransformElement",
    "SVGAnimatedAngle",
    "SVGAnimatedBoolean",
    "SVGAnimatedEnumeration",
    "SVGAnimatedInteger",
    "SVGAnimatedLength",
    "SVGAnimatedLengthList",
    "SVGAnimatedNumber",
    "SVGAnimatedNumberList",
    "SVGAnimatedPathData",
    "SVGAnimatedPoints",
    "SVGAnimatedPreserveAspectRatio",
    "SVGAnimatedRect",
    "SVGAnimatedString",
    "SVGAnimatedTransformList",
    "SVGAnimationElement",
    "SVGCSSRule",
    "SVGCircleElement",
    "SVGClipPathElement",
    "SVGColor",
    "SVGColorProfileElement",
    "SVGColorProfileRule",
    "SVGComponentTransferFunctionElement",
    "SVGCursorElement",
    "SVGDefsElement",
    "SVGDescElement",
    "SVGDocument",
    "SVGElement",
    "SVGElementInstance",
    "SVGElementInstanceList",
    "SVGEllipseElement",
    "SVGExternalResourcesRequired",
    "SVGFEBlendElement",
    "SVGFEColorMatrixElement",
    "SVGFEComponentTransferElement",
    "SVGFECompositeElement",
    "SVGFEConvolveMatrixElement",
    "SVGFEDiffuseLightingElement",
    "SVGFEDisplacementMapElement",
    "SVGFEDistantLightElement",
    "SVGFEFloodElement",
    "SVGFEFuncAElement",
    "SVGFEFuncBElement",
    "SVGFEFuncGElement",
    "SVGFEFuncRElement",
    "SVGFEGaussianBlurElement",
    "SVGFEImageElement",
    "SVGFEMergeElement",
    "SVGFEMergeNodeElement",
    "SVGFEMorphologyElement",
    "SVGFEOffsetElement",
    "SVGFEPointLightElement",
    "SVGFESpecularLightingElement",
    "SVGFESpotLightElement",
    "SVGFETileElement",
    "SVGFETurbulenceElement",
    "SVGFilterElement",
    "SVGFilterPrimitiveStandardAttributes",
    "SVGFitToViewBox",
    "SVGFontElement",
    "SVGFontFaceElement",
    "SVGFontFaceFormatElement",
    "SVGFontFaceNameElement",
    "SVGFontFaceSrcElement",
    "SVGFontFaceUriElement",
    "SVGForeignObjectElement",
    "SVGGElement",
    "SVGGlyphElement",
    "SVGGlyphRefElement",
    "SVGGradientElement",
    "SVGHKernElement",
    "SVGICCColor",
    "SVGImageElement",
    "SVGLangSpace",
    "SVGLength",
    "SVGLengthList",
    "SVGLineElement",
    "SVGLinearGradientElement",
    "SVGLocatable",
    "SVGMPathElement",
    "SVGMarkerElement",
    "SVGMaskElement",
    "SVGMatrix",
    "SVGMetadataElement",
    "SVGMissingGlyphElement",
    "SVGNumber",
    "SVGNumberList",
    "SVGPaint",
    "SVGPathElement",
    "SVGPathSeg",
    "SVGPathSegArcAbs",
    "SVGPathSegArcRel",
    "SVGPathSegClosePath",
    "SVGPathSegCurvetoCubicAbs",
    "SVGPathSegCurvetoCubicRel",
    "SVGPathSegCurvetoCubicSmoothAbs",
    "SVGPathSegCurvetoCubicSmoothRel",
    "SVGPathSegCurvetoQuadraticAbs",
    "SVGPathSegCurvetoQuadraticRel",
    "SVGPathSegCurvetoQuadraticSmoothAbs",
    "SVGPathSegCurvetoQuadraticSmoothRel",
    "SVGPathSegLinetoAbs",
    "SVGPathSegLinetoHorizontalAbs",
    "SVGPathSegLinetoHorizontalRel",
    "SVGPathSegLinetoRel",
    "SVGPathSegLinetoVerticalAbs",
    "SVGPathSegLinetoVerticalRel",
    "SVGPathSegList",
    "SVGPathSegMovetoAbs",
    "SVGPathSegMovetoRel",
    "SVGPatternElement",
    "SVGPoint",
    "SVGPointList",
    "SVGPolygonElement",
    "SVGPolylineElement",
    "SVGPreserveAspectRatio",
    "SVGRadialGradientElement",
    "SVGRect",
    "SVGRectElement",
    "SVGRenderingIntent",
    "SVGSVGElement",
    "SVGScriptElement",
    "SVGSetElement",
    "SVGStopElement",
    "SVGStringList",
    "SVGStylable",
    "SVGStyleElement",
    "SVGSwitchElement",
    "SVGSymbolElement",
    "SVGTRefElement",
    "SVGTSpanElement",
    "SVGTests",
    "SVGTextContentElement",
    "SVGTextElement",
    "SVGTextPathElement",
    "SVGTextPositioningElement",
    "SVGTitleElement",
    "SVGTransform",
    "SVGTransformList",
    "SVGTransformable",
    "SVGURIReference",
    "SVGUnitTypes",
    "SVGUseElement",
    "SVGVKernElement",
    "SVGViewElement",
    "SVGViewSpec",
    "SVGZoomAndPan",
    "TimeEvent",
    "top",
    "Uint16Array",
    "Uint32Array",
    "Uint8Array",
    "Uint8ClampedArray",
    "WebSocket",
    "window",
    "Worker",
    "XMLHttpRequest",
    "XMLSerializer",
    "XPathEvaluator",
    "XPathException",
    "XPathExpression",
    "XPathNamespace",
    "XPathNSResolver",
    "XPathResult",
];