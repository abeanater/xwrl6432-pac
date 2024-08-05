#[doc = "Register `DCEST1I_SW` reader"]
pub type R = crate::R<Dcest1iSwSpec>;
#[doc = "Register `DCEST1I_SW` writer"]
pub type W = crate::W<Dcest1iSwSpec>;
#[doc = "Field `DCEST1I_SW` reader - 23:0\\]
This register holds the software programmed dc value I to be subtracted from incoming sample for bcnt = 0."]
pub type Dcest1iSwR = crate::FieldReader<u32>;
#[doc = "Field `DCEST1I_SW` writer - 23:0\\]
This register holds the software programmed dc value I to be subtracted from incoming sample for bcnt = 0."]
pub type Dcest1iSwW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
This register holds the software programmed dc value I to be subtracted from incoming sample for bcnt = 0."]
    #[inline(always)]
    pub fn dcest1i_sw(&self) -> Dcest1iSwR {
        Dcest1iSwR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
This register holds the software programmed dc value I to be subtracted from incoming sample for bcnt = 0."]
    #[inline(always)]
    #[must_use]
    pub fn dcest1i_sw(&mut self) -> Dcest1iSwW<Dcest1iSwSpec> {
        Dcest1iSwW::new(self, 0)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<Dcest1iSwSpec> {
        Nu1W::new(self, 24)
    }
}
#[doc = "DCEST1I_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`dcest1i_sw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcest1i_sw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dcest1iSwSpec;
impl crate::RegisterSpec for Dcest1iSwSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcest1i_sw::R`](R) reader structure"]
impl crate::Readable for Dcest1iSwSpec {}
#[doc = "`write(|w| ..)` method takes [`dcest1i_sw::W`](W) writer structure"]
impl crate::Writable for Dcest1iSwSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCEST1I_SW to value 0"]
impl crate::Resettable for Dcest1iSwSpec {
    const RESET_VALUE: u32 = 0;
}
