# Example HTTP Req/Res program

As an example of the concepts we have seen so far, lets write a program that needs to perform an HTTP Request, expects an HTTP response from the webservice it accesses and then perform some logic on that response.

On an effect system like `Fx.go`, we *do not* directly contact external services, we just express our need to perform such requests and we expect a `Handler` to actually decide how and when such request should be performed (if any).


```go
package http_example

// Notice we do not import any HTTP library, just effects.
import (
	"testing"
	fx "github.com/vic/fx.go"
)

// For simplicity our request is just an string: An URL.
type HttpRq string

// For simplicity out response is just an string: The response body.
type HttpRs string

// Type of the effect-request function. 
// This will be implemented by some handler to provide actual responses.
//
// fx.Nil in the result type indicates that our Http ability does
// not requires any other ability.
type HttpFn func(HttpRq) fx.Fx[fx.Nil, HttpRs]

// Type of the Http Handler that discharges effect requirements
type HttpHn = func(fx.Fx[fx.And[HttpFn, fx.Nil], HttpRs]) fx.Fx[fx.Nil, HttpRs]

// Type of the Http Ability: the handler and aditional abilities.
type HttpAb = fx.And[HttpHn, fx.Nil]

// An http effect that produces V
type HttpFx[V any] = fx.Fx[HttpAb, V]

// An effect of HTTP GET requests.
func Get(url HttpRq) HttpFx[HttpRs] {
	return fx.Handle[HttpHn](url)
}

// A program that computes the respose length of https://example.org
func Program() HttpFx[int] {
	return fx.Map(Get("https://example.org"), func(r HttpRs) int {
		return len(r)
	})
}

func TestProgram(t *testing.T) {
	var httpHandler HttpHn = fx.Handler(func(r HttpRq) fx.Fx[fx.Nil, HttpRs] {
		mock := HttpRs("example")
		return fx.Pure(&mock)
	})
	var provided fx.Fx[fx.Nil, int] = fx.ProvideLeft(Program(), httpHandler)
	var result int = fx.Eval(provided)
	if result != len("example") {
		t.Errorf("Unexpected result %v", result)
	}
}

```
