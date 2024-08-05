#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `ECC_EN` reader - 0:0\\]
TI Internal : Enable ECC"]
pub type EccEnR = crate::BitReader;
#[doc = "Field `ECC_EN` writer - 0:0\\]
TI Internal : Enable ECC"]
pub type EccEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECC_CHK` reader - 1:1\\]
TI Internal : Enable ECC check"]
pub type EccChkR = crate::BitReader;
#[doc = "Field `ECC_CHK` writer - 1:1\\]
TI Internal : Enable ECC check"]
pub type EccChkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EN_RMW` reader - 2:2\\]
TI Internal : Enable rmw"]
pub type EnRmwR = crate::BitReader;
#[doc = "Field `EN_RMW` writer - 2:2\\]
TI Internal : Enable rmw"]
pub type EnRmwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_SEC` reader - 3:3\\]
TI Internal : Force Single Bit Error"]
pub type ForceSecR = crate::BitReader;
#[doc = "Field `FORCE_SEC` writer - 3:3\\]
TI Internal : Force Single Bit Error"]
pub type ForceSecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_DED` reader - 4:4\\]
TI Internal : Force Double Bit Error"]
pub type ForceDedR = crate::BitReader;
#[doc = "Field `FORCE_DED` writer - 4:4\\]
TI Internal : Force Double Bit Error"]
pub type ForceDedW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCE_N_ROW` reader - 5:5\\]
TI Internal : Force Error on any RAM read"]
pub type ForceNRowR = crate::BitReader;
#[doc = "Field `FORCE_N_ROW` writer - 5:5\\]
TI Internal : Force Error on any RAM read"]
pub type ForceNRowW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERROR_ONCE` reader - 6:6\\]
TI Internal : Force Error only once"]
pub type ErrorOnceR = crate::BitReader;
#[doc = "Field `ERROR_ONCE` writer - 6:6\\]
TI Internal : Force Error only once"]
pub type ErrorOnceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHECK PARITY` reader - 7:7\\]
TI Internal : Check Parity"]
pub type CheckparityR = crate::BitReader;
#[doc = "Field `CHECK PARITY` writer - 7:7\\]
TI Internal : Check Parity"]
pub type CheckparityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHECK TIMEOUT` reader - 8:8\\]
TI Internal : Check timeout"]
pub type ChecktimeoutR = crate::BitReader;
#[doc = "Field `CHECK TIMEOUT` writer - 8:8\\]
TI Internal : Check timeout"]
pub type ChecktimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU3` reader - 31:9\\]
TI Internal : Reserved"]
pub type Nu3R = crate::FieldReader<u32>;
#[doc = "Field `NU3` writer - 31:9\\]
TI Internal : Reserved"]
pub type Nu3W<'a, REG> = crate::FieldWriter<'a, REG, 23, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
TI Internal : Enable ECC"]
    #[inline(always)]
    pub fn ecc_en(&self) -> EccEnR {
        EccEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
TI Internal : Enable ECC check"]
    #[inline(always)]
    pub fn ecc_chk(&self) -> EccChkR {
        EccChkR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
TI Internal : Enable rmw"]
    #[inline(always)]
    pub fn en_rmw(&self) -> EnRmwR {
        EnRmwR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 3:3\\]
TI Internal : Force Single Bit Error"]
    #[inline(always)]
    pub fn force_sec(&self) -> ForceSecR {
        ForceSecR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - 4:4\\]
TI Internal : Force Double Bit Error"]
    #[inline(always)]
    pub fn force_ded(&self) -> ForceDedR {
        ForceDedR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - 5:5\\]
TI Internal : Force Error on any RAM read"]
    #[inline(always)]
    pub fn force_n_row(&self) -> ForceNRowR {
        ForceNRowR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 6:6\\]
TI Internal : Force Error only once"]
    #[inline(always)]
    pub fn error_once(&self) -> ErrorOnceR {
        ErrorOnceR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
TI Internal : Check Parity"]
    #[inline(always)]
    pub fn checkparity(&self) -> CheckparityR {
        CheckparityR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - 8:8\\]
TI Internal : Check timeout"]
    #[inline(always)]
    pub fn checktimeout(&self) -> ChecktimeoutR {
        ChecktimeoutR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:31 - 31:9\\]
TI Internal : Reserved"]
    #[inline(always)]
    pub fn nu3(&self) -> Nu3R {
        Nu3R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
TI Internal : Enable ECC"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_en(&mut self) -> EccEnW<CtrlSpec> {
        EccEnW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
TI Internal : Enable ECC check"]
    #[inline(always)]
    #[must_use]
    pub fn ecc_chk(&mut self) -> EccChkW<CtrlSpec> {
        EccChkW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
TI Internal : Enable rmw"]
    #[inline(always)]
    #[must_use]
    pub fn en_rmw(&mut self) -> EnRmwW<CtrlSpec> {
        EnRmwW::new(self, 2)
    }
    #[doc = "Bit 3 - 3:3\\]
TI Internal : Force Single Bit Error"]
    #[inline(always)]
    #[must_use]
    pub fn force_sec(&mut self) -> ForceSecW<CtrlSpec> {
        ForceSecW::new(self, 3)
    }
    #[doc = "Bit 4 - 4:4\\]
TI Internal : Force Double Bit Error"]
    #[inline(always)]
    #[must_use]
    pub fn force_ded(&mut self) -> ForceDedW<CtrlSpec> {
        ForceDedW::new(self, 4)
    }
    #[doc = "Bit 5 - 5:5\\]
TI Internal : Force Error on any RAM read"]
    #[inline(always)]
    #[must_use]
    pub fn force_n_row(&mut self) -> ForceNRowW<CtrlSpec> {
        ForceNRowW::new(self, 5)
    }
    #[doc = "Bit 6 - 6:6\\]
TI Internal : Force Error only once"]
    #[inline(always)]
    #[must_use]
    pub fn error_once(&mut self) -> ErrorOnceW<CtrlSpec> {
        ErrorOnceW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
TI Internal : Check Parity"]
    #[inline(always)]
    #[must_use]
    pub fn checkparity(&mut self) -> CheckparityW<CtrlSpec> {
        CheckparityW::new(self, 7)
    }
    #[doc = "Bit 8 - 8:8\\]
TI Internal : Check timeout"]
    #[inline(always)]
    #[must_use]
    pub fn checktimeout(&mut self) -> ChecktimeoutW<CtrlSpec> {
        ChecktimeoutW::new(self, 8)
    }
    #[doc = "Bits 9:31 - 31:9\\]
TI Internal : Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu3(&mut self) -> Nu3W<CtrlSpec> {
        Nu3W::new(self, 9)
    }
}
#[doc = "CTRL\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CTRL to value 0"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0;
}
