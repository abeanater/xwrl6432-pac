#[doc = "Register `DCEST2I` reader"]
pub type R = crate::R<Dcest2iSpec>;
#[doc = "Register `DCEST2I` writer"]
pub type W = crate::W<Dcest2iSpec>;
#[doc = "Field `DCEST2I` reader - 23:0\\]
This register holds the estimated dc value I to be subtracted from incoming sample for bcnt =1 ."]
pub type Dcest2iR = crate::FieldReader<u32>;
#[doc = "Field `DCEST2I` writer - 23:0\\]
This register holds the estimated dc value I to be subtracted from incoming sample for bcnt =1 ."]
pub type Dcest2iW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
This register holds the estimated dc value I to be subtracted from incoming sample for bcnt =1 ."]
    #[inline(always)]
    pub fn dcest2i(&self) -> Dcest2iR {
        Dcest2iR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
This register holds the estimated dc value I to be subtracted from incoming sample for bcnt =1 ."]
    #[inline(always)]
    #[must_use]
    pub fn dcest2i(&mut self) -> Dcest2iW<Dcest2iSpec> {
        Dcest2iW::new(self, 0)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<Dcest2iSpec> {
        Nu1W::new(self, 24)
    }
}
#[doc = "DCEST2I\n\nYou can [`read`](crate::Reg::read) this register and get [`dcest2i::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcest2i::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dcest2iSpec;
impl crate::RegisterSpec for Dcest2iSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcest2i::R`](R) reader structure"]
impl crate::Readable for Dcest2iSpec {}
#[doc = "`write(|w| ..)` method takes [`dcest2i::W`](W) writer structure"]
impl crate::Writable for Dcest2iSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCEST2I to value 0"]
impl crate::Resettable for Dcest2iSpec {
    const RESET_VALUE: u32 = 0;
}
