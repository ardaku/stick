// "stick" crate - Licensed under the MIT LICENSE
//  * Copyright (c) 2017-2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>

{
	fn remapper(input: (usize, ::Input)) -> (usize, ::Input) {
		(input.0, match input.1 {
			::Input::Move(x, y) => {
				::Input::Move(x.min(1.0).max(-1.0),
					y.min(1.0).max(-1.0))
			}
			::Input::Camera(x, y) => {
				::Input::Camera(x.min(1.0).max(-1.0),
					y.min(1.0).max(-1.0))
			}
			::Input::ThrottleL(x) => {
				::Input::ThrottleL(x.min(1.0).max(-1.0))
			}
			::Input::ThrottleR(x) => {
				::Input::ThrottleR(x.min(1.0).max(-1.0))
			}
			a => a
		})
	}

	::Remapper::new(0, remapper)
}
