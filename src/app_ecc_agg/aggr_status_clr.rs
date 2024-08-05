#[doc = "Register `AGGR_STATUS_CLR` reader"]
pub type R = crate::R<AggrStatusClrSpec>;
#[doc = "Register `AGGR_STATUS_CLR` writer"]
pub type W = crate::W<AggrStatusClrSpec>;
#[doc = "Field `PARITY` reader - 1:0\\]
interrupt status clear for parity errors - (RW decr)"]
pub type ParityR = crate::FieldReader;
#[doc = "Field `PARITY` writer - 1:0\\]
interrupt status clear for parity errors - (RW decr)"]
pub type ParityW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIMEOUT` reader - 3:2\\]
interrupt status clear for svbus timeout errors - (RW decr)"]
pub type TimeoutR = crate::FieldReader;
#[doc = "Field `TIMEOUT` writer - 3:2\\]
interrupt status clear for svbus timeout errors - (RW decr)"]
pub type TimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RES18` reader - 31:4\\]
RESERVE FIELD"]
pub type Res18R = crate::FieldReader<u32>;
#[doc = "Field `RES18` writer - 31:4\\]
RESERVE FIELD"]
pub type Res18W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:1 - 1:0\\]
interrupt status clear for parity errors - (RW decr)"]
    #[inline(always)]
    pub fn parity(&self) -> ParityR {
        ParityR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - 3:2\\]
interrupt status clear for svbus timeout errors - (RW decr)"]
    #[inline(always)]
    pub fn timeout(&self) -> TimeoutR {
        TimeoutR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:31 - 31:4\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res18(&self) -> Res18R {
        Res18R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:1 - 1:0\\]
interrupt status clear for parity errors - (RW decr)"]
    #[inline(always)]
    #[must_use]
    pub fn parity(&mut self) -> ParityW<AggrStatusClrSpec> {
        ParityW::new(self, 0)
    }
    #[doc = "Bits 2:3 - 3:2\\]
interrupt status clear for svbus timeout errors - (RW decr)"]
    #[inline(always)]
    #[must_use]
    pub fn timeout(&mut self) -> TimeoutW<AggrStatusClrSpec> {
        TimeoutW::new(self, 2)
    }
    #[doc = "Bits 4:31 - 31:4\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res18(&mut self) -> Res18W<AggrStatusClrSpec> {
        Res18W::new(self, 4)
    }
}
#[doc = "AGGR interrupt status clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`aggr_status_clr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aggr_status_clr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AggrStatusClrSpec;
impl crate::RegisterSpec for AggrStatusClrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aggr_status_clr::R`](R) reader structure"]
impl crate::Readable for AggrStatusClrSpec {}
#[doc = "`write(|w| ..)` method takes [`aggr_status_clr::W`](W) writer structure"]
impl crate::Writable for AggrStatusClrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AGGR_STATUS_CLR to value 0"]
impl crate::Resettable for AggrStatusClrSpec {
    const RESET_VALUE: u32 = 0;
}
