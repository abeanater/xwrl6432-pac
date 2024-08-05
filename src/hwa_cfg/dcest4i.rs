#[doc = "Register `DCEST4I` reader"]
pub type R = crate::R<Dcest4iSpec>;
#[doc = "Register `DCEST4I` writer"]
pub type W = crate::W<Dcest4iSpec>;
#[doc = "Field `DCEST4I` reader - 23:0\\]
This register holds the estimated dc value I to be subtracted from incoming sample for bcnt =3."]
pub type Dcest4iR = crate::FieldReader<u32>;
#[doc = "Field `DCEST4I` writer - 23:0\\]
This register holds the estimated dc value I to be subtracted from incoming sample for bcnt =3."]
pub type Dcest4iW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
This register holds the estimated dc value I to be subtracted from incoming sample for bcnt =3."]
    #[inline(always)]
    pub fn dcest4i(&self) -> Dcest4iR {
        Dcest4iR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
This register holds the estimated dc value I to be subtracted from incoming sample for bcnt =3."]
    #[inline(always)]
    #[must_use]
    pub fn dcest4i(&mut self) -> Dcest4iW<Dcest4iSpec> {
        Dcest4iW::new(self, 0)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<Dcest4iSpec> {
        Nu1W::new(self, 24)
    }
}
#[doc = "DCEST4I\n\nYou can [`read`](crate::Reg::read) this register and get [`dcest4i::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcest4i::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dcest4iSpec;
impl crate::RegisterSpec for Dcest4iSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcest4i::R`](R) reader structure"]
impl crate::Readable for Dcest4iSpec {}
#[doc = "`write(|w| ..)` method takes [`dcest4i::W`](W) writer structure"]
impl crate::Writable for Dcest4iSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCEST4I to value 0"]
impl crate::Resettable for Dcest4iSpec {
    const RESET_VALUE: u32 = 0;
}
