#[doc = "Register `DCEST6Q` reader"]
pub type R = crate::R<Dcest6qSpec>;
#[doc = "Register `DCEST6Q` writer"]
pub type W = crate::W<Dcest6qSpec>;
#[doc = "Field `DCEST6Q` reader - 23:0\\]
This register holds the estimated dc value Q to be subtracted from incoming sample for bcnt =5 ."]
pub type Dcest6qR = crate::FieldReader<u32>;
#[doc = "Field `DCEST6Q` writer - 23:0\\]
This register holds the estimated dc value Q to be subtracted from incoming sample for bcnt =5 ."]
pub type Dcest6qW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
This register holds the estimated dc value Q to be subtracted from incoming sample for bcnt =5 ."]
    #[inline(always)]
    pub fn dcest6q(&self) -> Dcest6qR {
        Dcest6qR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
This register holds the estimated dc value Q to be subtracted from incoming sample for bcnt =5 ."]
    #[inline(always)]
    #[must_use]
    pub fn dcest6q(&mut self) -> Dcest6qW<Dcest6qSpec> {
        Dcest6qW::new(self, 0)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<Dcest6qSpec> {
        Nu1W::new(self, 24)
    }
}
#[doc = "DCEST6Q\n\nYou can [`read`](crate::Reg::read) this register and get [`dcest6q::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcest6q::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dcest6qSpec;
impl crate::RegisterSpec for Dcest6qSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcest6q::R`](R) reader structure"]
impl crate::Readable for Dcest6qSpec {}
#[doc = "`write(|w| ..)` method takes [`dcest6q::W`](W) writer structure"]
impl crate::Writable for Dcest6qSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCEST6Q to value 0"]
impl crate::Resettable for Dcest6qSpec {
    const RESET_VALUE: u32 = 0;
}
