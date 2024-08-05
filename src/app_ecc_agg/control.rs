#[doc = "Register `CONTROL` reader"]
pub type R = crate::R<ControlSpec>;
#[doc = "Register `CONTROL` writer"]
pub type W = crate::W<ControlSpec>;
#[doc = "Field `ECC_ENABLE` reader - 0:0\\]
Enable ECC - (RW )"]
pub type EccEnableR = crate::BitReader;
#[doc = "Field `ECC_ENABLE` writer - 0:0\\]
Enable ECC - (RW )"]
pub type EccEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_CHECK` reader - 1:1\\]
Enable ECC check - (RW )"]
pub type EccCheckR = crate::BitReader;
#[doc = "Field `ECC_CHECK` writer - 1:1\\]
Enable ECC check - (RW )"]
pub type EccCheckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE_RMW` reader - 2:2\\]
Enable rmw - (RW )"]
pub type EnableRmwR = crate::BitReader;
#[doc = "Field `ENABLE_RMW` writer - 2:2\\]
Enable rmw - (RW )"]
pub type EnableRmwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_SEC` reader - 3:3\\]
Force Single Bit Error - (RW )"]
pub type ForceSecR = crate::BitReader;
#[doc = "Field `FORCE_SEC` writer - 3:3\\]
Force Single Bit Error - (RW )"]
pub type ForceSecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_DED` reader - 4:4\\]
Force Double Bit Error - (RW )"]
pub type ForceDedR = crate::BitReader;
#[doc = "Field `FORCE_DED` writer - 4:4\\]
Force Double Bit Error - (RW )"]
pub type ForceDedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_N_ROW` reader - 5:5\\]
Force Error on any RAM read - (RW )"]
pub type ForceNRowR = crate::BitReader;
#[doc = "Field `FORCE_N_ROW` writer - 5:5\\]
Force Error on any RAM read - (RW )"]
pub type ForceNRowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERROR_ONCE` reader - 6:6\\]
Force Error only once - (RW )"]
pub type ErrorOnceR = crate::BitReader;
#[doc = "Field `ERROR_ONCE` writer - 6:6\\]
Force Error only once - (RW )"]
pub type ErrorOnceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHECK_PARITY` reader - 7:7\\]
check for parity errors - (RW )"]
pub type CheckParityR = crate::BitReader;
#[doc = "Field `CHECK_PARITY` writer - 7:7\\]
check for parity errors - (RW )"]
pub type CheckParityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHECK_SVBUS_TIMEOUT` reader - 8:8\\]
check for svbus timeout errors - (RW )"]
pub type CheckSvbusTimeoutR = crate::BitReader;
#[doc = "Field `CHECK_SVBUS_TIMEOUT` writer - 8:8\\]
check for svbus timeout errors - (RW )"]
pub type CheckSvbusTimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES4` reader - 31:9\\]
RESERVE FIELD"]
pub type Res4R = crate::FieldReader<u32>;
#[doc = "Field `RES4` writer - 31:9\\]
RESERVE FIELD"]
pub type Res4W<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Enable ECC - (RW )"]
    #[inline(always)]
    pub fn ecc_enable(&self) -> EccEnableR {
        EccEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enable ECC check - (RW )"]
    #[inline(always)]
    pub fn ecc_check(&self) -> EccCheckR {
        EccCheckR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
Enable rmw - (RW )"]
    #[inline(always)]
    pub fn enable_rmw(&self) -> EnableRmwR {
        EnableRmwR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
Force Single Bit Error - (RW )"]
    #[inline(always)]
    pub fn force_sec(&self) -> ForceSecR {
        ForceSecR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
Force Double Bit Error - (RW )"]
    #[inline(always)]
    pub fn force_ded(&self) -> ForceDedR {
        ForceDedR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
Force Error on any RAM read - (RW )"]
    #[inline(always)]
    pub fn force_n_row(&self) -> ForceNRowR {
        ForceNRowR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
Force Error only once - (RW )"]
    #[inline(always)]
    pub fn error_once(&self) -> ErrorOnceR {
        ErrorOnceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
check for parity errors - (RW )"]
    #[inline(always)]
    pub fn check_parity(&self) -> CheckParityR {
        CheckParityR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
check for svbus timeout errors - (RW )"]
    #[inline(always)]
    pub fn check_svbus_timeout(&self) -> CheckSvbusTimeoutR {
        CheckSvbusTimeoutR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:31 - 31:9\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res4(&self) -> Res4R {
        Res4R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Enable ECC - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_enable(&mut self) -> EccEnableW<ControlSpec> {
        EccEnableW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Enable ECC check - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_check(&mut self) -> EccCheckW<ControlSpec> {
        EccCheckW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
Enable rmw - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn enable_rmw(&mut self) -> EnableRmwW<ControlSpec> {
        EnableRmwW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
Force Single Bit Error - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn force_sec(&mut self) -> ForceSecW<ControlSpec> {
        ForceSecW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
Force Double Bit Error - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn force_ded(&mut self) -> ForceDedW<ControlSpec> {
        ForceDedW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
Force Error on any RAM read - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn force_n_row(&mut self) -> ForceNRowW<ControlSpec> {
        ForceNRowW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
Force Error only once - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn error_once(&mut self) -> ErrorOnceW<ControlSpec> {
        ErrorOnceW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
check for parity errors - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn check_parity(&mut self) -> CheckParityW<ControlSpec> {
        CheckParityW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
check for svbus timeout errors - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn check_svbus_timeout(&mut self) -> CheckSvbusTimeoutW<ControlSpec> {
        CheckSvbusTimeoutW::new(self, 8)
    }
    #[doc = "Bits 9:31 - 31:9\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res4(&mut self) -> Res4W<ControlSpec> {
        Res4W::new(self, 9)
    }
}
#[doc = "ECC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`control::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`control::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ControlSpec;
impl crate::RegisterSpec for ControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`control::R`](R) reader structure"]
impl crate::Readable for ControlSpec {}
#[doc = "`write(|w| ..)` method takes [`control::W`](W) writer structure"]
impl crate::Writable for ControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONTROL to value 0"]
impl crate::Resettable for ControlSpec {
    const RESET_VALUE: u32 = 0;
}
