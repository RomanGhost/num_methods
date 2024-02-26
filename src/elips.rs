pub fn elipsoid64(eps:f64) ->f64{
    let mut count=0;
    let mut new_eps:f64 = eps;
    let mut new_eps1:f64 = 0.0;

    loop{
        count+=1;
        new_eps /= 2.0;
        new_eps1 = new_eps + 1.0;
        if new_eps1 <= 1.0{
            break;
        }
    }
    println!("{}", count);
    new_eps
}

pub fn elipsoid32(eps:f32) ->f32{
    let mut new_eps:f32 = eps;
    let mut new_eps1:f32 = 0.0;

    loop{
        new_eps /= 2.0;
        new_eps1 = new_eps + 1.0;
        if new_eps1 <= 1.0
        {
            break;
        }
    }
    new_eps
}