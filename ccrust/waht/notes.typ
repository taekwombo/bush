#import "@preview/cetz:0.0.1"

= Neural Network
#link("https://www.youtube.com/playlist?list=PLZHQObOWTQDNU6R1_67000Dx_ZCJB-3pi", text(blue)[3B1B Neural Networks])

Networks composed of layers of neurons and connections. Neuron is a function that outputs a number.
Layer is a group of neurons.

#set align(center)
#cetz.canvas({
    import cetz.draw:*

    circle((0,4), radius: 0.5, name: "l00")
    content((), [$a#sub[0]#super[(0)]$], anchor: "center")
    circle((0,0), radius: 0.5, name: "l01")
    content((), [$a#sub[1]#super[(0)]$], anchor: "center")

    circle((4,4), radius: 0.5, name: "l10")
    content((), [$a#sub[0]#super[(1)]$], anchor: "center")
    circle((4,0), radius: 0.5, name: "l11")
    content((), [$a#sub[1]#super[(1)]$], anchor: "center")

    let hu = v => cetz.vector.add(v, (0, 0.2))
    let hd = v => cetz.vector.add(v, (0, -0.2))

    line("l00.right", "l10.left", stroke: rgb(100, 100, 100), mark: (end: ">"))
    translate((-1.5, 0.1))
    content((), [$w#sub[00]#super[(1)]$], anchor: "bottom")
    translate((1.5, -0.1))

    line("l00.right", (hu, "l11.left"), stroke: rgb(100, 100, 100), mark: (end: ">"))
    translate((-2.5, 1.2))
    content((), [$w#sub[10]#super[(1)]$], anchor: "bottom")
    translate((2.5, -1.2))

    line("l01.right", (hd, "l10.left"), stroke: rgb(120, 120, 220), mark: (end: ">"))
    translate((-2.5, -1.5))
    content((), [$w#sub[01]#super[(1)]$], anchor: "bottom")
    translate((2.5, 1.5))

    line("l01.right", "l11.left", stroke: rgb(120, 120, 220), mark: (end: ">"))
    translate((-1.5, 0.1))
    content((), [$w#sub[11]#super[(1)]$], anchor: "bottom")
    translate((1.5, -0.1))

    circle((8,2), radius: 0.5, name: "l20")
    content((), [$a#sub[0]#super[(2)]$], anchor: "center")

    line("l10.right", (hu, "l20.left"), stroke: rgb(100, 100, 100), mark: (end: ">"))
    translate((-1.5, 1.4))
    content((), [$w#sub[00]#super[(2)]$], anchor: "bottom")
    translate((1.5, -1.4))

    line("l11.right", (hd, "l20.left"), stroke: rgb(100, 100, 100), mark: (end: ">"))
    translate((-1.5, -1.5))
    content((), [$w#sub[01]#super[(2)]$], anchor: "bottom")
    translate((1.5, 1.5))
})

#set align(left)

- $a#sub[n]#super[(0)]$     - input layer
- $a#sub[n]#super[(1..m)]$  - hidden to output layers
- $m$                       - number of layers
- $S#super[(k)]$            - number of activations in $k#super[th]$ layer
- $w#sub[ij]#super[(k)]$    - weight of connection between $a#sub[i]#super[(k)]$ and $a#sub[j]#super[(k-1)]$
- $b#sub[j]#super[(k)]$     - bias of the $i$#super[th] neuron from $k$#super[th] layer.

== Calculating activation of a neuron

Multiply all previous layer activations by connection weight and add bias.

$
                  c &= S#super[(k)] - 1 \
                  n &= S#super[(k-1)] - 1 \
a#sub[i]#super[(k)] &= sigma(a#sub[0]#super[(k-1)]w#sub[i 0]#super[(k)] + a#sub[1]#super[(k-1)]w#sub[i 1]#super[(k)] + ... + a#sub[n]#super[(k-1)]w#sub[i n]#super[(k)]) \
                    &= sigma((sum_(h = 0)^(n) w#sub[i h]#super[(k)]a#sub[h]#super[(k-1)]) + b#sub[i]) \
                    &= sigma(
                        mat(delim: "[", a#sub[0]#super[(k-1)], a#sub[1]#super[(k-1)], ..., a#sub[n]#super[(k-1)])
                        dot
                        vec(delim: "[", w#sub[i 0]#super[(k)], w#sub[i 1]#super[(k)], dots.v, w#sub[i n]#super[(k)])
                        +
                        mat(delim: "[", b#sub[i]#super[(k)])
                    )
$
This can be simplified for the whole layer.
$
vec(delim: "[", a#sub[0]#super[(k)], a#sub[1]#super[(k)], dots.v, a#sub[c]#super[(k)]) = 
sigma (
    mat(
        delim: "[",
        a#sub[0]#super[(k-1)],
        a#sub[1]#super[(k-1)],
        dots.h,
        a#sub[n]#super[(k-1)]
    )
    dot
    mat(
        delim: "[",
        w#sub[0 0]#super[(k)], w#sub[1 0]#super[(k)], ..., w#sub[c 0]#super[(k)],;
        w#sub[0 1]#super[(k)], w#sub[1 1]#super[(k)], ..., w#sub[c 1]#super[(k)],;
        dots.v, dots.v, dots.down, dots.v;
        w#sub[0 n]#super[(k)], w#sub[1 n]#super[(k)], ..., w#sub[c n]#super[(k)];
    )
    +
    vec(
        delim: "[",
        b#sub[0]#super[(k)],
        b#sub[1]#super[(k)],
        dots.v,
        b#sub[c]#super[(k)]
    )
)
$

#pagebreak()
== Cost function

- $y#sub[t i]$ - expected activation value at $i#super[th]$ neuron of last layer for $t#super[th]$ training sample
- $T$ - number of training samples
- $C = a#super[(k)] - y$ - cost for single neuron output layer
- $a#sub[i]#super[(k)] - y#sub[i]$ - difference between actual and expected value for $i#super[th]$ neuron
$
n &= S#super[(k)] - 1 \
C &= 1/T sum_(t = 0)^(T-1)(sum_(i = 0)^(n) (a#sub[i]#super[(k)] - y#sub[t i])^2)
$

==== Single output two inputs

#let di = [a#sub[0]#super[(1)]]
#let zi = [a#sub[0]#super[(0)]w#sub[0 0]#super[(1)] + a#sub[1]#super[(0)]w#sub[0 1]#super[(1)] + b#sub[0]#super[(1)]]
#let z  = [z]
$
#z &= #zi \
#di &= sigma(#z) \
\
diff#sub[w#sub[0 0]#super[(1)]]a#sub[0]#super[(1)] &= diff sigma(#z) \
                                        &= [#link("https://hausetutorials.netlify.app/posts/2019-12-01-neural-networks-deriving-the-sigmoid-derivative", text(blue)[derivative of sigmoid])] \
                                        &= sigma(#z)(1 - sigma(#z)) diff(#zi) \
                                        &= sigma(#z)(1 - sigma(#z)) a#sub[0]#super[(0)] \
                                        &= #di (1 - #di) a#sub[0]#super[(0)] \
                                        \
diff#sub[w#sub[0 1]#super[(1)]]a#sub[0]#super[(1)] &= #di (1 - #di) a#sub[1]#super[(0)] \
diff#sub[b#sub[0]#super[(1)]]a#sub[0]#super[(1)]   &= #di (1 - #di) \
diff#sub[a#sub[0]#super[(1)]]a#sub[0]#super[(1)] &= #di (1 - #di) w#sub[0 0]#super[(1)] \
diff#sub[a#sub[1]#super[(1)]]a#sub[0]#super[(1)] &= #di (1 - #di) w#sub[0 1]#super[(1)] \
$
For single test sample:
$
C#super[(1)] &= (#di - y)^2 \

diff#sub[w#sub[0 0]#super[(1)]]C#super[(1)] &= diff (#di - y)^2 \
                                 &= 2(#di - y) diff (#di - y) \
                                 &= 2(#di - y) diff #di \
                                 &= 2(#di - y) #di\(1 - #di) a#sub[0]#super[(0)]\
diff#sub[w#sub[0 1]#super[(1)]]C#super[(1)] &= diff (#di - y)^2 \
                                 &= 2(#di - y) #di\(1 - #di) a#sub[1]#super[(0)]\
diff#sub[b#sub[0]#super[(1)]]C#super[(1)] &= diff (#di - y)^2 \
                               &= 2(#di - y) #di\(1 - #di) \
diff#sub[a#sub[0]#super[(0)]]C#super[(1)] &= diff (#di - y)^2 \
                               &= 2(#di - y) #di\(1 - #di) w#sub[0 0]#super[(2)] \
diff#sub[a#sub[1]#super[(0)]]C#super[(1)] &= diff (#di - y)^2 \
                               &= 2(#di - y) #di\(1 - #di) w#sub[0 1]#super[(2)] \
$
Let $(1)#super[st]$ layer be a hidden layer,
$y#sub[0]#super[(1)]$ be the expected value for first neuron of input layer. The expected value of this neuron is a difference between its value and partial derivative from next layer.
$
y#sub[0]#super[(1)] &= a#sub[0]#super[(1)] - diff#sub[a#sub[0]#super[(1)]]C#super[(2)] \
diff#sub[w#sub[0 0]#super[(1)]]C#super[(1)] &= diff (a#sub[0]#super[(1)] - y#sub[0]#super[(1)])^2 \
                               &= 2(a#sub[0]#super[(1)] - y#sub[0]#super[(0)]) diff (a#sub[0]#super[(1)] - y#sub[0]#super[(0)])\
                               &= 2(a#sub[0]#super[(1)] - y#sub[0]#super[(0)]) diff a#sub[0]#super[(1)] \
                               &= 2(a#sub[0]#super[(1)] - (a#sub[0]#super[(1)] - diff#sub[a#sub[0]#super[(2)]]C#super[(2)])) diff a#sub[0]#super[(1)] \
                               &= 2(diff#sub[a#sub[0]#super[(2)]]C#super[(2)]) diff a#sub[0]#super[(1)] \
                               &= 2(diff#sub[a#sub[0]#super[(2)]]C#super[(2)])#di\(1 - #di) a#sub[0]#super[(0)] \

diff#sub[w#sub[0 1]#super[(1)]]C#super[(1)] &= diff (a#sub[0]#super[(1)] - y#sub[0]#super[(1)])^2 \
                                            &= 2(diff#sub[a#sub[0]#super[(2)]]C#super[(2)])#di\(1 - #di) a#sub[1]#super[(0)] \

diff#sub[b#sub[0]#super[(1)]]C#super[(1)]   &= diff (a#sub[0]#super[(1)] - y#sub[0]#super[(1)])^2 \
                                            &= 2(diff#sub[a#sub[0]#super[(2)]]C#super[(2)])#di\(1 - #di) \
$
With variable layer index and neuron index:
$

C#sub[i]#super[(k)] &= (a#sub[i]#super[(k)] - y#sub[i]#super[(k)]) ^ 2
                    = (diff#sub[a#sub[i]#super[(k)]]C#super[(k+1)]) ^ 2 \
diff#sub[b#sub[i]#super[(k)]]C#sub[i]#super[(k)]        &= 2(diff#sub[a#sub[i]#super[(k)]]C#super[(k+1)])a#sub[i]#super[(k)]\(1 - a#sub[i]#super[(k)]) \
diff#sub[w#sub[i n]#super[(k)]]C#sub[i]#super[(k)]      &= 2(diff#sub[a#sub[i]#super[(k)]]C#super[(k+1)])a#sub[i]#super[(k)]\(1 - a#sub[i]#super[(k)])a#sub[n]#super[(k-1)] \
// diff#sub[a#sub[n]#super[(k-1)]]C#sub[i]#super[(k)] &= 2(diff#sub[a#sub[i]#super[(k)]]C#super[(k+1)])a#sub[i]#super[(k)]\(1 - a#sub[i]#super[(k)])w#sub[i n]#super[(k)] \
diff#sub[a#sub[i]#super[(k-1)]]C#super[(k)]             &= sum_(j=0)^(S#super[(k)]-1)(2(diff#sub[a#sub[i]#super[(k)]]C#super[(k+1)])a#sub[j]#super[(k)]\(1 - a#sub[j]#super[(k)])w#sub[j i]#super[(k)])\
$
Since single previous layer neuron influences all next layer neurons, the derivative for the change in such neuron activation must be summed over next layer.
