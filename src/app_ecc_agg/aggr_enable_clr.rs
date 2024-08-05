#[doc = "Register `AGGR_ENABLE_CLR` reader"]
pub type R = crate::R<AggrEnableClrSpec>;
#[doc = "Register `AGGR_ENABLE_CLR` writer"]
pub type W = crate::W<AggrEnableClrSpec>;
#[doc = "Field `PARITY` reader - 0:0\\]
interrupt enable clear for parity errors - (RW )"]
pub type ParityR = crate::BitReader;
#[doc = "Field `PARITY` writer - 0:0\\]
interrupt enable clear for parity errors - (RW )"]
pub type ParityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMEOUT` reader - 1:1\\]
interrupt enable clear for svbus timeout errors - (RW )"]
pub type TimeoutR = crate::BitReader;
#[doc = "Field `TIMEOUT` writer - 1:1\\]
interrupt enable clear for svbus timeout errors - (RW )"]
pub type TimeoutW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES16` reader - 31:2\\]
RESERVE FIELD"]
pub type Res16R = crate::FieldReader<u32>;
#[doc = "Field `RES16` writer - 31:2\\]
RESERVE FIELD"]
pub type Res16W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
interrupt enable clear for parity errors - (RW )"]
    #[inline(always)]
    pub fn parity(&self) -> ParityR {
        ParityR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
interrupt enable clear for svbus timeout errors - (RW )"]
    #[inline(always)]
    pub fn timeout(&self) -> TimeoutR {
        TimeoutR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res16(&self) -> Res16R {
        Res16R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
interrupt enable clear for parity errors - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn parity(&mut self) -> ParityW<AggrEnableClrSpec> {
        ParityW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
interrupt enable clear for svbus timeout errors - (RW )"]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TimeoutW<AggrEnableClrSpec> {
        TimeoutW::new(self, 1)
    }
    #[doc = "Bits 2:31 - 31:2\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res16(&mut self) -> Res16W<AggrEnableClrSpec> {
        Res16W::new(self, 2)
    }
}
#[doc = "AGGR interrupt enable clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aggr_enable_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aggr_enable_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AggrEnableClrSpec;
impl crate::RegisterSpec for AggrEnableClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aggr_enable_clr::R`](R) reader structure"]
impl crate::Readable for AggrEnableClrSpec {}
#[doc = "`write(|w| ..)` method takes [`aggr_enable_clr::W`](W) writer structure"]
impl crate::Writable for AggrEnableClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AGGR_ENABLE_CLR to value 0"]
impl crate::Resettable for AggrEnableClrSpec {
    const RESET_VALUE: u32 = 0;
}
