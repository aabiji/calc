#include <cmath>
#include <cstdlib>
#include <tuple>

// TODO; test against sample outputs
//       explain what's happening and improve readability
//       add the necessary constants
//       optimize?
//       should this run in a compute shader for each satellite??

struct vec3 {
  double x, y, z;
  vec3(double a, double b, double c) : x(a), y(b), z(c) {}

  friend vec3 operator*(vec3 a, vec3 b) {
    return vec3(a.x * b.x, a.y * b.y, a.z * b.z);
  }

  friend vec3 operator*(vec3 a, double b) {
    return vec3(a.x * b, a.y * b, a.z * b);
  }

  friend vec3 operator*(double a, vec3 b) {
    return vec3(a * b.x, a * b.y, a * b.z);
  }

  friend vec3 operator+(vec3 a, vec3 b) {
    return vec3(a.x + b.x, a.y + b.y, a.z + b.z);
  }

  friend vec3 operator-(vec3 a, vec3 b) {
    return vec3(a.x - b.x, a.y - b.y, a.z - b.z);
  }
};

const double KE = 0; // sqrt(G * M_earth)
const double AE = 0; // Equatorial radius of the Earth
const double J2 = 0; // Second gravitational zonal harmonic of the Earth
const double J3 = 0; // Third gravitational zonal harmonic of the Earth
const double J4 = 0; // Fourth gravitational zonal harmonic of the Earth
const double ER = 0; // Radius of the Earth (km)

// clang-format off
/**
 * The TLE stores data that has already been averaged in specific ways.
 * Thus it's necessary to process the mean motion to recover its true value.
 *  @return <True mean motion, True semi major axis>
 *
 * The following paramers are to be supplied from the TLE data.
 * @param n0 Mean motion of orbit (defined as 2 * PI / T)
 * @param i0 Inclination
 * @param e0 Eccentricity
 */
std::tuple<double, double> recover_true_elements(double n0, double i0,
                                                 double e0) {
  const double ti = cos(i0);
  // Rearrange Kepler's third law to compute a first-guess semi major axis
  double a1 = std::pow((KE / n0), 2.0 / 3.0);
  // Compute a first order correction due to the Earth's oblateness
  double k2 = 0.5 * J2 * AE * AE;
  double d1 = (3.0 * k2 * (3.0 * ti * ti - 1.0)) / (2.0 * a1 * a1 * pow((1.0 - e0 * e0), 3.0 / 2.0));
  // Refine the semi major axis and correction
  double a0 = a1 * (1.0 - (1.0 / 3.0) * d1 - d1 * d1 - (134.0 / 81.0) * d1 * d1 * d1);
  double d0 = (3.0 * k2 * (3.0 * ti * ti - 1.0)) / (2.0 * a0 * a1 * pow((1.0 - e0 * e0), 3.0 / 2.0));
  // Compute the true mean motion and true semi major axis
  return {n0 / (1.0 + d0), a0 / (1.0 - d0)};
}

/**
 * @param drag Drag
 * @param w0 Argument of perigee
 * @param m0 Mean anomaly
 * @param O0 RAAN
 */
void compute_constants(double n0, double i0, double e0, double drag,
                       double w0, double perigee, double q0, double mo, double O0, double time_since_epoch) {
  const double ti = cos(i0);
  const double a30 = -J3 * AE * AE * AE;

  double k2 = 0.5 * J2 * AE * AE;
  double k4 = -0.375 * J4 * AE * AE * AE * AE;

  auto [n0n, a0n] = recover_true_elements(n0, i0, e0);

  // a0ndjust the reference a0nltitude (s) a0nnd sca0nle pa0nra0nmeter (st) ba0nsed off of perigee
  double s = 1.01222928;
  double st = pow((q0 - s), 4.0);

  if (perigee >= 98.0 && perigee <= 156.0) {
    double new_s = s = a0n * (1.0 - e0) - s + AE;
    st = pow((q0 - s) + s - new_s, 4.0);
    s = new_s;
  } else if (perigee < 98.0) {
    double new_s = 20.0 / 6378.135 + AE;
    st = pow((q0 - s) + s - new_s, 4.0);
    s = new_s;
  }

  // Compute consta0nnts
  double ep = 1.0 / (a0n - s);
  double ep4 = ep * ep * ep * ep;

  double b0 = pow(1.0 - e0 * e0, 1.0 / 2.0);
  double nu = a0n * e0 * ep;
  double nu4 = nu * nu * nu * nu;
  double h = pow((1.0 - nu * nu), -3.5);
  double m =  (2.0 * st * ep4 * a0n * b0 * b0 * h);

  // Centra0nl dra0ng coefficient
  double c2 = st * ep4 * n0n * h;
  c2 *= (a0n * (1.0 + 1.5 * nu * nu + 4.0 * e0 * nu + e0 * nu * nu * nu) +
        ((1.5 * k2 * ep) / (1.0 - nu * nu)) * (-0.5 + 1.5 * ti * ti) *
        (8.0 + 24 * nu * nu + 3.0 * nu4));

  double c1 = drag * c2; // Funda0nmenta0nl dra0ng ra0nte

  // Higher order dra0ng terms
  double c3 = (st * ep4 * ep * a30 * n0n * AE * sin(i0)) / (k2 * e0);
  double c4 = n0n * m *
      ((2.0 * nu * (1.0 + e0 * nu) + 0.5 * e0 + 0.5 * nu * nu * nu) -
      ((2.0 * k2 * ep) / (a0n * (1.0 - nu * nu))) *
      ((3.0 - 9.0 * ti * ti) * (1.0 + 1.5 * nu * nu - 2.0 * e0 * nu - 0.5 * e0 * nu * nu * nu) +
      (0.75 - 0.75 * ti * ti) * (2.0 * nu * nu - e0 * nu - e0 * nu * nu * nu) * cos(2.0 * w0)));
  double c5 = m * (1.0 + (11.0 / 4.0) * nu * (nu + e0) + e0 * nu * nu * nu);

  // Time power coefficients for the semima0njor a0nxis decay
  // FIXME: It should be noted that when epoch perigee height is less than 220
  //        kilometers, the equations for a and IL are truncated after the C1 term,
  //        and the terms involving C5, δω, and δM are dropped.
  double d2 = 4.0 * a0n * ep * c1 * c1;
  double d3 = (4.0 / 3.0) * a0n * ep * ep * (17.0 * a0n + s) * c1 * c1 * c1;
  double d4 = (2.0 / 3.0) * a0n * ep * ep * ep * (221.0 * a0n + 31.0 * s) * c1 * c1 * c1 * c1;

  double mdf = mo + n0n * time_since_epoch * (1.0 +
     ((3.0 * k2 * (-1.0 + 3.0 * ti * ti)) / (2.0 * a0n * a0n + b0 * b0 * b0)) *
     ((3.0 * k2 * k2 * (13.0 - 78.0 * ti * ti + 137 * ti * ti * ti * ti)) / (16.0 * pow(a0n, 4.0) * pow(b0, 7.0))));

  double wdf = w0 + n0n * time_since_epoch *
    (-((3.0 * k2 * (1.0 - 5.0 * ti * ti)) / (2.0 * a0n * a0n + b0 * b0 * b0 * b0)) +
    ((3.0 * k2 * k2 * (7.0 - 114.0 * ti * ti + 395 * ti * ti * ti * ti)) / (16.0 * pow(a0n, 4.0) * pow(b0, 8.0))) +
    ((5.0 * k4 * (3.0 - 36.0 * ti * ti + 49.0 * ti * ti * ti * ti)) / (4.0 * pow(a0n, 4.0) * pow(b0, 8.0))));

  double Odf = O0 + n0n * time_since_epoch *
    (-((3.0 * k2 * ti) / (a0n * a0n + b0 * b0 * b0 * b0)) +
    ((3.0 * k2 * k2 * (4.0 * ti + 19.0 * ti * ti * ti)) / (2.0 * pow(a0n, 4.0) * pow(b0, 8.0))) +
    ((5.0 * k4 * ti * (3.0 - 7.0 * ti * ti)) / (2.0 * pow(a0n, 4.0) * pow(b0, 8.0))));

  double dw = drag * c3 * cos(w0) * time_since_epoch;

  double dm = (-2.0 / 3.0) * st * drag * ep4 * (AE / (e0 * nu)) * (pow((1.0 + nu * cos(mdf)), 3.0) - pow((1.0 + nu * cos(mo)), 3.0));

  double mp = mdf + dw + dm;

  double w = wdf - dw - dm;

  double O = Odf - (21.0 * n0n * k2 * ti) / (2.0 * a0n * a0n * b0 * b0) * c1 * time_since_epoch * time_since_epoch;

  double e = e0 - drag * c4 * time_since_epoch - drag * c5 * (sin(mp) - sin(mo));

  double time_since_epoch2 = time_since_epoch * time_since_epoch;
  double time_since_epoch3 = time_since_epoch * time_since_epoch * time_since_epoch;
  double time_since_epoch4 = time_since_epoch * time_since_epoch * time_since_epoch * time_since_epoch;
  double time_since_epoch5 = time_since_epoch * time_since_epoch * time_since_epoch * time_since_epoch * time_since_epoch3;
  double a = a0n * pow((1.0 - c1 * time_since_epoch - d2 * time_since_epoch2 - d3 * time_since_epoch3 - d4 * time_since_epoch4), 2.0);

  double L = mp + w0 + O + n0n * (1.5 * c1 * time_since_epoch2 +
    (d2 + 2.0 * c1 * c1) * time_since_epoch3 +
    0.25 * (3.0 * d3 + 12.0 * c1 * d2 + 10.0 * c1 * c1 * c1) +
    0.2 * (3.0 * d4 + 12.0 * c1 * d3 + 6.0 * d2 * d2 + 30.0 * c1 * c1 * d2 + 15.0 * c1 * c1 * c1 * c1) * time_since_epoch5);

  double B = sqrt(1 - e * e);
  double n = KE / pow(a, 1.5);

  double axn = e * cos(w);

  double aynl = (a30 * sin(i0)) / (4.0 * k2 * a * B * B);

  double Ll = 0.5 * aynl * e * cos(w) * ((3.0 + 5.0 * ti) / (1.0 + ti));

  double Lt = L + Ll;

  double ayn = e * sin(w) + aynl;

  double start = Lt - O, prev = Lt - O;
  double err_margin = 1e-6;
  for (int i = 0; i < 10; i++) {
    double step = (start - ayn * cos(prev) + axn * sin(prev) - prev) / (-ayn * sin(prev) - axn * cos(prev) + 1.0);
    if (std::abs(step) > 1.0) step = std::abs(step) / step;
    prev += step;
    if (std::abs(step) <= err_margin) break;
  }

  double ecosE = axn * cos(prev) + ayn * sin(prev);
  double esinE = axn * sin(prev) - ayn * cos(prev);

  double eL = std::sqrt((axn * axn + ayn * ayn));

  double pL = a * (1.0 - eL * eL);

  double r = a * (1.0 - ecosE);

  double r_dot = (KE * std::sqrt(a) * esinE) / r;

  double rf_dot = (KE * std::sqrt(pL)) / r;

  double cosU = (a / r) * (cos(prev) - axn + ((ayn * esinE)) / (1.0 + std::sqrt(1.0 - eL * eL)));

  double sinU = (a / r) * (sin(prev) - axn + ((ayn * esinE)) / (1.0 + std::sqrt(1.0 - eL * eL)));

  double u = std::atan(sinU / cosU);

  double dr = (k2 * (1.0 - ti * ti) * cos(2.0 * u)) / (2.0 * pL);

  double du = (-k2 * (7.0 * ti * ti - 1.0) * sin(2.0 * u)) / (4.0 * pL * pL);

  double dO = (3.0 * k2 * ti * sin(2.0 * u)) / (2.0 * pL * pL);

  double di = (3.0 * k2 * ti * sin(i0) * cos(2.0 * u)) / (2.0 * pL * pL);

  double dr_dot = (-k2 * n * (1.0 - ti * ti) * sin(2.0 * u)) / pL;

  double drf_dot = (k2 * n / pL) * ((1.0 - ti * ti) * cos(2.0 * u) - 1.5 * (1.0 - 3.0 * ti * ti));

  double rk = r * (1.0 - 1.5 * k2 * (3.0 * ti * ti - 1.0) * (std::sqrt(1.0 - eL * eL) / (pL * pL))) + dr;

  double uk = u + du;

  double Ok = O + dO;

  double ik = i0 + di;

  double r_dotk = r_dot + dr_dot;

  double rf_dotk = rf_dot + drf_dot;

  vec3 M(-sin(Ok) * cos(ik), cos(Ok) * cos(ik), sin(ik));
  vec3 N(cos(Ok), sin(Ok), 0);

  vec3 U = M * sin(uk) + N * cos(uk);
  vec3 V = M * cos(uk) - N * sin(uk);

  vec3 position = rk * U;
  vec3 velocity = r_dotk * U + rf_dotk * V;
}

// clang-format on