/-
Copyright (c) 2026 Pythia contributors. All rights reserved.
Released under Apache 2.0 license as described in the file LICENSE.

# Fixed Income — complete toolkit

One import for bond pricing, yield curve construction, duration/convexity
risk, and interest rate models.

    import Pythia.Finance.FixedIncome

## Modules

* **Bond pricing:** zero-coupon, price-yield, yield from price
* **Yield curve:** bootstrap, forward rates, discount factors
* **Duration:** Macaulay duration, convexity, DV01
* **Interest rates:** compound interest, annuity factor, perpetuity
* **Rate models:** Vasicek short rate, Vasicek bond price
* **Forward:** forward price, forward rate parity, FX forward,
  continuous dividend forward
-/

import Pythia.Finance.BondPriceYield
import Pythia.Finance.BondZeroCoupon
import Pythia.Finance.YieldFromPrice
import Pythia.Finance.BootstrapYieldCurve
import Pythia.Finance.DiscountFactor
import Pythia.Finance.MacaulayDuration
import Pythia.Finance.ConvexityDuration
import Pythia.Finance.CompoundInterest
import Pythia.Finance.AnnuityFactor
import Pythia.Finance.Perpetuity
import Pythia.Finance.VasicekShortRate
import Pythia.Finance.VasicekBondPrice
import Pythia.Finance.ForwardPrice
import Pythia.Finance.ForwardRateParity
import Pythia.Finance.FxForward
import Pythia.Finance.ContinuousDividendForward
