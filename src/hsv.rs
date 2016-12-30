use super::rgba_module_base::Rgba;

#[inline(always)]
fn min3<'a, T>(a: &'a T, b: &'a T, c: &'a T) -> &'a T
    where T: ::std::cmp::PartialOrd
{
    let mut cm = a;

    if let ::std::cmp::Ordering::Less = cm.partial_cmp(b).unwrap() {
        cm = b;
    }

    if let ::std::cmp::Ordering::Less = cm.partial_cmp(c).unwrap() {
        cm = c;
    }

    cm
}

#[inline(always)]
fn max3<'a, T>(a: &'a T, b: &'a T, c: &'a T) -> &'a T
    where T: ::std::cmp::PartialOrd
{
    let mut cm = a;

    if let ::std::cmp::Ordering::Greater = cm.partial_cmp(b).unwrap() {
        cm = b;
    }

    if let ::std::cmp::Ordering::Greater = cm.partial_cmp(c).unwrap() {
        cm = c;
    }

    cm
}

pub fn rgba_to_hsv(col: &Rgba) -> Rgba {
    let mn = *min3(&col[0], &col[1], &col[2]);
    let mx = *max3(&col[0], &col[1], &col[2]);
    let dlt = mx - mn;
    let v = mx;

    let s = if mx != 0.0 {
        // s=dlt/mx;
        1.0 - (mn / mx)
    } else {
        return Rgba::with_all(0.0, 0.0, 0.0, col[3]);
    };

    let h = if col[0] == mx {
        // h=(col[1]-col[2])/dlt;
        ((60.0 * ((col[1] - col[2]) / dlt)) as i32 % 360) as f32
    } else if col[1] == mx {
        60.0 * ((col[2] - col[0]) / dlt) + 120.0
    } else {
        // h=4.0+(col[0]-col[1])/dlt;
        60.0 * ((col[0] - col[1]) / dlt) + 240.0
    };

    // h=h*60;
    // if(h<0) h=h+360.0;
    Rgba::with_all(h / 360.0, s, v, col[3])

    // 	float maxc=max3(c[0], c[1], c[2]);
    // float minc=min3(c[0], c[1], c[2]);
    // int h;
    // float s,v;
    // if(minc==maxc) h=0;
    // else if(maxc==c[0]) h=(int)(60.0 * ((c[1]-c[2]) / (maxc-minc))) % 360;
    // else if(maxc==c[1]) h=(int)(60.0 * ((c[2]-c[0]) / (maxc-minc))) +120;
    // else h=(int)(60.0 * ((c[0]-c[1]) / (maxc-minc))) +240;
    // v=maxc;
    // if(maxc==0.0) s=0.0;
    // else s=1.0-(minc/maxc);
    // hsv=CRGBAf((float)(h)/360.0, s, v, c[3]);
}

pub fn hsv_to_rgba(hsv: &Rgba) -> Rgba {
    // hsv[0]=hsv[0]*360.0;
    // int hi = ((int)(floorf(hsv[0] / 60.0)) % 6);
    // float f=(hsv[0]/60.0) - floorf(hsv[0]/60.0);
    // float p=hsv[2]*(1.0-hsv[1]);
    // float q=hsv[2]*(1.0-(f*hsv[1]));
    // float t=hsv[2]*(1.0-((1.0-f)*hsv[1]));
    // float v=hsv[2];
    //
    // switch(hi)
    // {
    // case 0: col=CRGBAf(v,t,p,hsv[3]); break;
    // case 1: col=CRGBAf(q,v,p,hsv[3]); break;
    // case 2: col=CRGBAf(p,v,t,hsv[3]); break;
    // case 3: col=CRGBAf(p,q,v,hsv[3]); break;
    // case 4: col=CRGBAf(t,p,v,hsv[3]); break;
    // case 5: col=CRGBAf(v,p,q,hsv[3]); break;
    // default: break;
    // }
    //

    let s = hsv[1];
    let v = hsv[2];

	if s == 0.0 {
        return Rgba::with_all(v, v, v, hsv[3]);
    }

    let h = (hsv[0] * 360.0) / 60.0;
    
    let i = h.floor();
    let f = h - i;
    let p = v * (1.0 - s);
    let q = v * (1.0 - s * f);
    let t = v * (1.0 - s * (1.0 - f));

    let (r, g, b) = if i == 0.0 {
        (v, t, p)
    } else if i == 1.0 {
        (q, v, p)
    } else if i == 2.0 {
        (p, v, t)
    } else if i == 3.0 {
        (p, q, v)
    } else if i == 4.0 {
        (t, p, v)
    } else {
        (v, p, q)
    };

    Rgba::with_all(r, g, b, hsv[3])
}
