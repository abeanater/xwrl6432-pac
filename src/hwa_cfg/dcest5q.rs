#[doc = "Register `DCEST5Q` reader"]
pub type R = crate::R<Dcest5qSpec>;
#[doc = "Register `DCEST5Q` writer"]
pub type W = crate::W<Dcest5qSpec>;
#[doc = "Field `DCEST5Q` reader - 23:0\\]
This register holds the estimated dc value Q to be subtracted from incoming sample for bcnt =4 ."]
pub type Dcest5qR = crate::FieldReader<u32>;
#[doc = "Field `DCEST5Q` writer - 23:0\\]
This register holds the estimated dc value Q to be subtracted from incoming sample for bcnt =4 ."]
pub type Dcest5qW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
This register holds the estimated dc value Q to be subtracted from incoming sample for bcnt =4 ."]
    #[inline(always)]
    pub fn dcest5q(&self) -> Dcest5qR {
        Dcest5qR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
This register holds the estimated dc value Q to be subtracted from incoming sample for bcnt =4 ."]
    #[inline(always)]
    #[must_use]
    pub fn dcest5q(&mut self) -> Dcest5qW<Dcest5qSpec> {
        Dcest5qW::new(self, 0)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<Dcest5qSpec> {
        Nu1W::new(self, 24)
    }
}
#[doc = "DCEST5Q\n\nYou can [`read`](crate::Reg::read) this register and get [`dcest5q::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcest5q::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dcest5qSpec;
impl crate::RegisterSpec for Dcest5qSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcest5q::R`](R) reader structure"]
impl crate::Readable for Dcest5qSpec {}
#[doc = "`write(|w| ..)` method takes [`dcest5q::W`](W) writer structure"]
impl crate::Writable for Dcest5qSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCEST5Q to value 0"]
impl crate::Resettable for Dcest5qSpec {
    const RESET_VALUE: u32 = 0;
}
