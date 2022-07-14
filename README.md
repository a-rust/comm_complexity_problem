# Problem
This is a simple implementation of the communication complexity of equality problem. This is supposed to be educational.

## Setup
Alice and Bob are each given an tuple with n elements from some field $F$.

Let $a_n$ be Alice's tuple, and $b_n$ be Bob's tuple.

How can Alice check if $a_n = b_n$?
Bob could send Alice $b_n$, and Alice could just check every input against $a_n$. But is there a faster way that requires checking less steps?

Consider a function $\phi$ that maps the elements of the n-tuple to a corresponding dense polynomial $p(x)\in F[x]$, whose coefficients are exactly the inputs of some n-tuple.
* For example, $\phi((5, 3, -7)) = 5 + 3x -7x^2$.

Suppose Bob sends $(r, \phi({b_n})(r))$ to Alice for some random $r\in F$.
* Does it follow thar $\phi({b_n})(r) = \phi({a_n})(r)$ implies $b_n = a_n$?

The answer really depends on the field:
* Let $p(x) = 1 + 2x + 2x^2 + x^3 \in F_3[x]$, and $q(x) = 1 + x + 2x^2 + 2x^2 \in F_3[x]$.
* $(1, 2, 2, 1) \neq (1, 1, 2, 2)$, yet $p(1) = 0 = q(1)$.

It turns out that given $p(x) \neq q(x)$, $Pr[p(r) = q(r)] \leq \frac{n}{|F|}$ (check out https://en.wikipedia.org/wiki/Communication_complexity).

Going back to the example above with polynomials in $F_3[x]$, $Pr[p(r) = q(r)] \leq \frac{n}{|F|} = \frac{4}{3}$.

So finite fields are probably not the best choice for this setting. On the other hand, any extension of $Q$ should make it difficult for 2 different polynomials evaluated at the sane input to have the same output.
* Given $p(x) \neq q(x)$, $Pr[p(r) = q(r)] \leq \frac{n}{|Q|}$, which would be very small.

Thus, if $a_n$ and $b_n$ contain elements in some extension $E$ of $Q$, and $r\in E$, then Alice could check $\phi(b_n)(r) = \phi(a_n)(r)$ to determine (with high probability) if $a_n = b_n$ or not.

## Running


## Example
~~~
    let alice = Character {
        name: "Alice".to_string(),
        polynomial: vec![10, 20, 30, 40, 80, 100],
        random_input: 4,
    };

    let bob = Character {
        name: "Bob".to_string(),
        polynomial: vec![10, 20, 30, 40, 80, 100],
        random_input: 4,
    };

    assert_eq!(bob.compare(alice), true);
~~~