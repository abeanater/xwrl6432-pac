#[doc = "Register `DCEST6I_SW` reader"]
pub type R = crate::R<Dcest6iSwSpec>;
#[doc = "Register `DCEST6I_SW` writer"]
pub type W = crate::W<Dcest6iSwSpec>;
#[doc = "Field `DCEST6I_SW` reader - 23:0\\]
This register holds the software programmed dc value I to be subtracted from incoming sample for bcnt = 5."]
pub type Dcest6iSwR = crate::FieldReader<u32>;
#[doc = "Field `DCEST6I_SW` writer - 23:0\\]
This register holds the software programmed dc value I to be subtracted from incoming sample for bcnt = 5."]
pub type Dcest6iSwW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
This register holds the software programmed dc value I to be subtracted from incoming sample for bcnt = 5."]
    #[inline(always)]
    pub fn dcest6i_sw(&self) -> Dcest6iSwR {
        Dcest6iSwR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
This register holds the software programmed dc value I to be subtracted from incoming sample for bcnt = 5."]
    #[inline(always)]
    #[must_use]
    pub fn dcest6i_sw(&mut self) -> Dcest6iSwW<Dcest6iSwSpec> {
        Dcest6iSwW::new(self, 0)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<Dcest6iSwSpec> {
        Nu1W::new(self, 24)
    }
}
#[doc = "DCEST6I_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`dcest6i_sw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcest6i_sw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Dcest6iSwSpec;
impl crate::RegisterSpec for Dcest6iSwSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcest6i_sw::R`](R) reader structure"]
impl crate::Readable for Dcest6iSwSpec {}
#[doc = "`write(|w| ..)` method takes [`dcest6i_sw::W`](W) writer structure"]
impl crate::Writable for Dcest6iSwSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DCEST6I_SW to value 0"]
impl crate::Resettable for Dcest6iSwSpec {
    const RESET_VALUE: u32 = 0;
}
