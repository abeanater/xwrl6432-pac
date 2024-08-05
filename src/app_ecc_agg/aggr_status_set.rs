#[doc = "Register `AGGR_STATUS_SET` reader"]
pub type R = crate::R<AggrStatusSetSpec>;
#[doc = "Register `AGGR_STATUS_SET` writer"]
pub type W = crate::W<AggrStatusSetSpec>;
#[doc = "Field `PARITY` reader - 1:0\\]
interrupt status set for parity errors - (RW incr)"]
pub type ParityR = crate::FieldReader;
#[doc = "Field `PARITY` writer - 1:0\\]
interrupt status set for parity errors - (RW incr)"]
pub type ParityW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIMEOUT` reader - 3:2\\]
interrupt status set for svbus timeout errors - (RW incr)"]
pub type TimeoutR = crate::FieldReader;
#[doc = "Field `TIMEOUT` writer - 3:2\\]
interrupt status set for svbus timeout errors - (RW incr)"]
pub type TimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RES17` reader - 31:4\\]
RESERVE FIELD"]
pub type Res17R = crate::FieldReader<u32>;
#[doc = "Field `RES17` writer - 31:4\\]
RESERVE FIELD"]
pub type Res17W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
interrupt status set for parity errors - (RW incr)"]
    #[inline(always)]
    pub fn parity(&self) -> ParityR {
        ParityR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
interrupt status set for svbus timeout errors - (RW incr)"]
    #[inline(always)]
    pub fn timeout(&self) -> TimeoutR {
        TimeoutR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:31 - 31:4\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res17(&self) -> Res17R {
        Res17R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
interrupt status set for parity errors - (RW incr)"]
    #[inline(always)]
    #[must_use]
    pub fn parity(&mut self) -> ParityW<AggrStatusSetSpec> {
        ParityW::new(self, 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
interrupt status set for svbus timeout errors - (RW incr)"]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TimeoutW<AggrStatusSetSpec> {
        TimeoutW::new(self, 2)
    }
    #[doc = "Bits 4:31 - 31:4\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res17(&mut self) -> Res17W<AggrStatusSetSpec> {
        Res17W::new(self, 4)
    }
}
#[doc = "AGGR interrupt status set Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aggr_status_set::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aggr_status_set::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AggrStatusSetSpec;
impl crate::RegisterSpec for AggrStatusSetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aggr_status_set::R`](R) reader structure"]
impl crate::Readable for AggrStatusSetSpec {}
#[doc = "`write(|w| ..)` method takes [`aggr_status_set::W`](W) writer structure"]
impl crate::Writable for AggrStatusSetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AGGR_STATUS_SET to value 0"]
impl crate::Resettable for AggrStatusSetSpec {
    const RESET_VALUE: u32 = 0;
}
