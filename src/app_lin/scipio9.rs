#[doc = "Register `SCIPIO9` reader"]
pub type R = crate::R<Scipio9Spec>;
#[doc = "Register `SCIPIO9` writer"]
pub type W = crate::W<Scipio9Spec>;
#[doc = "Field `CLKSL` reader - 0:0\\]
Reserved"]
pub type ClkslR = crate::BitReader;
#[doc = "Field `CLKSL` writer - 0:0\\]
Reserved"]
pub type ClkslW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSL` reader - 1:1\\]
SCIRX PIN value"]
pub type RxslR = crate::BitReader;
#[doc = "Field `RXSL` writer - 1:1\\]
SCIRX PIN value"]
pub type RxslW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXSL` reader - 2:2\\]
SCITX PIN value"]
pub type TxslR = crate::BitReader;
#[doc = "Field `TXSL` writer - 2:2\\]
SCITX PIN value"]
pub type TxslW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved1` reader - 31:16\\]
Reserved"]
pub type Reserved1R = crate::FieldReader<u16>;
#[doc = "Field `Reserved1` writer - 31:16\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Reserved"]
    #[inline(always)]
    pub fn clksl(&self) -> ClkslR {
        ClkslR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
SCIRX PIN value"]
    #[inline(always)]
    pub fn rxsl(&self) -> RxslR {
        RxslR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 2:2\\]
SCITX PIN value"]
    #[inline(always)]
    pub fn txsl(&self) -> TxslR {
        TxslR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn clksl(&mut self) -> ClkslW<Scipio9Spec> {
        ClkslW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
SCIRX PIN value"]
    #[inline(always)]
    #[must_use]
    pub fn rxsl(&mut self) -> RxslW<Scipio9Spec> {
        RxslW::new(self, 1)
    }
    #[doc = "Bit 2 - 2:2\\]
SCITX PIN value"]
    #[inline(always)]
    #[must_use]
    pub fn txsl(&mut self) -> TxslW<Scipio9Spec> {
        TxslW::new(self, 2)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<Scipio9Spec> {
        Reserved1W::new(self, 16)
    }
}
#[doc = "Couldn't find this register in spec. But it's mentioned in RTL.\n\nYou can [`read`](crate::Reg::read) this register and get [`scipio9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scipio9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Scipio9Spec;
impl crate::RegisterSpec for Scipio9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scipio9::R`](R) reader structure"]
impl crate::Readable for Scipio9Spec {}
#[doc = "`write(|w| ..)` method takes [`scipio9::W`](W) writer structure"]
impl crate::Writable for Scipio9Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCIPIO9 to value 0"]
impl crate::Resettable for Scipio9Spec {
    const RESET_VALUE: u32 = 0;
}
