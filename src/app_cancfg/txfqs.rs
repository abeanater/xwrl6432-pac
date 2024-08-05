#[doc = "Register `TXFQS` reader"]
pub type R = crate::R<TxfqsSpec>;
#[doc = "Register `TXFQS` writer"]
pub type W = crate::W<TxfqsSpec>;
#[doc = "Field `TFFL` reader - 5:0\\]
Tx FIFO Free Level"]
pub type TfflR = crate::FieldReader;
#[doc = "Field `TFFL` writer - 5:0\\]
Tx FIFO Free Level"]
pub type TfflW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `NU62` reader - 7:6\\]
Reserved"]
pub type Nu62R = crate::FieldReader;
#[doc = "Field `NU62` writer - 7:6\\]
Reserved"]
pub type Nu62W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TFGI` reader - 12:8\\]
Tx Queue Get Index"]
pub type TfgiR = crate::FieldReader;
#[doc = "Field `TFGI` writer - 12:8\\]
Tx Queue Get Index"]
pub type TfgiW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `NU63` reader - 15:13\\]
Reserved"]
pub type Nu63R = crate::FieldReader;
#[doc = "Field `NU63` writer - 15:13\\]
Reserved"]
pub type Nu63W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TFQPI` reader - 20:16\\]
Tx FIFO/Queue Put Index"]
pub type TfqpiR = crate::FieldReader;
#[doc = "Field `TFQPI` writer - 20:16\\]
Tx FIFO/Queue Put Index"]
pub type TfqpiW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TFQF` reader - 21:21\\]
Tx FIFO/Queue Full"]
pub type TfqfR = crate::BitReader;
#[doc = "Field `TFQF` writer - 21:21\\]
Tx FIFO/Queue Full"]
pub type TfqfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU64` reader - 31:22\\]
Reserved"]
pub type Nu64R = crate::FieldReader<u16>;
#[doc = "Field `NU64` writer - 31:22\\]
Reserved"]
pub type Nu64W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Tx FIFO Free Level"]
    #[inline(always)]
    pub fn tffl(&self) -> TfflR {
        TfflR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Reserved"]
    #[inline(always)]
    pub fn nu62(&self) -> Nu62R {
        Nu62R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Tx Queue Get Index"]
    #[inline(always)]
    pub fn tfgi(&self) -> TfgiR {
        TfgiR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 13:15 - 15:13\\]
Reserved"]
    #[inline(always)]
    pub fn nu63(&self) -> Nu63R {
        Nu63R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Tx FIFO/Queue Put Index"]
    #[inline(always)]
    pub fn tfqpi(&self) -> TfqpiR {
        TfqpiR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 21 - 21:21\\]
Tx FIFO/Queue Full"]
    #[inline(always)]
    pub fn tfqf(&self) -> TfqfR {
        TfqfR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:31 - 31:22\\]
Reserved"]
    #[inline(always)]
    pub fn nu64(&self) -> Nu64R {
        Nu64R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Tx FIFO Free Level"]
    #[inline(always)]
    #[must_use]
    pub fn tffl(&mut self) -> TfflW<TxfqsSpec> {
        TfflW::new(self, 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu62(&mut self) -> Nu62W<TxfqsSpec> {
        Nu62W::new(self, 6)
    }
    #[doc = "Bits 8:12 - 12:8\\]
Tx Queue Get Index"]
    #[inline(always)]
    #[must_use]
    pub fn tfgi(&mut self) -> TfgiW<TxfqsSpec> {
        TfgiW::new(self, 8)
    }
    #[doc = "Bits 13:15 - 15:13\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu63(&mut self) -> Nu63W<TxfqsSpec> {
        Nu63W::new(self, 13)
    }
    #[doc = "Bits 16:20 - 20:16\\]
Tx FIFO/Queue Put Index"]
    #[inline(always)]
    #[must_use]
    pub fn tfqpi(&mut self) -> TfqpiW<TxfqsSpec> {
        TfqpiW::new(self, 16)
    }
    #[doc = "Bit 21 - 21:21\\]
Tx FIFO/Queue Full"]
    #[inline(always)]
    #[must_use]
    pub fn tfqf(&mut self) -> TfqfW<TxfqsSpec> {
        TfqfW::new(self, 21)
    }
    #[doc = "Bits 22:31 - 31:22\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu64(&mut self) -> Nu64W<TxfqsSpec> {
        Nu64W::new(self, 22)
    }
}
#[doc = "TXFQS\n\nYou can [`read`](crate::Reg::read) this register and get [`txfqs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txfqs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxfqsSpec;
impl crate::RegisterSpec for TxfqsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txfqs::R`](R) reader structure"]
impl crate::Readable for TxfqsSpec {}
#[doc = "`write(|w| ..)` method takes [`txfqs::W`](W) writer structure"]
impl crate::Writable for TxfqsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXFQS to value 0"]
impl crate::Resettable for TxfqsSpec {
    const RESET_VALUE: u32 = 0;
}
